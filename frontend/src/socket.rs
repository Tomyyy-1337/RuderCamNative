use std::time::Duration;

use nsh_core::prelude::*;
use shared_data::Message;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

const SOCKET_URL: &str = "ws://127.0.0.1:3000/ws";
const RETRY_DELAY: Duration = Duration::from_secs(3);

fn apply_message(message: Message, camera_interface: Signal<shared_data::CameraInterface>) {
    match message {
        Message::CameraInterfaceUpdate(update) => {
            camera_interface.set(update);
        }
    }
}

enum SocketEvent {
    Opened,
    Closed,
    Message(Message),
}

/// Keeps a connection to the server open, retrying on disconnect. `connected`
/// follows the connection state and each server message goes to `apply_message`.
pub fn start(connected: Signal<bool>, camera_interface: Signal<shared_data::CameraInterface>) {
    let (events, mut received) = unbounded_channel();

    spawn(async move {
        while let Some(event) = received.recv().await {
            match event {
                SocketEvent::Opened => connected.set(true),
                SocketEvent::Closed => connected.set(false),
                SocketEvent::Message(message) => apply_message(message, camera_interface.clone()),
            }
        }
    });

    spawn(keep_connected(events));
}

async fn keep_connected(events: UnboundedSender<SocketEvent>) {
    loop {
        if let Err(error) = run_session(events.clone()).await {
            log::warn!("socket: {error}");
        }
        let _ = events.send(SocketEvent::Closed);
        timer(RETRY_DELAY).await;
    }
}

fn forward_text(text: &str, events: &UnboundedSender<SocketEvent>) {
    match serde_json::from_str::<Message>(text) {
        Ok(message) => {
            let _ = events.send(SocketEvent::Message(message));
        }
        Err(error) => log::warn!("socket: ignored message: {error}"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn run_session(events: UnboundedSender<SocketEvent>) -> Result<(), String> {
    match background(read_socket(events)).await {
        Ok(result) => result,
        Err(error) => Err(error.to_string()),
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn read_socket(events: UnboundedSender<SocketEvent>) -> Result<(), String> {
    use futures_util::StreamExt;
    use tokio_tungstenite::tungstenite::Message as Frame;

    let (mut socket, _) = tokio_tungstenite::connect_async(SOCKET_URL)
        .await
        .map_err(|error| error.to_string())?;
    let _ = events.send(SocketEvent::Opened);

    while let Some(frame) = socket.next().await {
        match frame.map_err(|error| error.to_string())? {
            Frame::Text(text) => forward_text(&text, &events),
            Frame::Close(_) => break,
            _ => {}
        }
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
async fn run_session(events: UnboundedSender<SocketEvent>) -> Result<(), String> {
    use wasm_bindgen::{JsCast, closure::Closure};
    use web_sys::{CloseEvent, MessageEvent, WebSocket};

    let socket = WebSocket::new(SOCKET_URL).map_err(|error| format!("{error:?}"))?;
    let (closed_tx, closed_rx) = tokio::sync::oneshot::channel::<()>();

    let open_events = events.clone();
    let on_open = Closure::<dyn FnMut()>::new(move || {
        let _ = open_events.send(SocketEvent::Opened);
    });
    let message_events = events.clone();
    let on_message = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        if let Some(text) = event.data().as_string() {
            forward_text(&text, &message_events);
        }
    });
    let on_close = Closure::once(move |_: CloseEvent| {
        let _ = closed_tx.send(());
    });

    socket.set_onopen(Some(on_open.as_ref().unchecked_ref()));
    socket.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    socket.set_onclose(Some(on_close.as_ref().unchecked_ref()));

    let _ = closed_rx.await;
    Ok(())
}
