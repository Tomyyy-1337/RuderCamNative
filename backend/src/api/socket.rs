use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{StreamExt, stream::select_all};
use shared_data::{BatteryState, CameraInterface, Message as SharedMessage};
use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    camera_interface: watch::Receiver<CameraInterface>,
    battery_state: watch::Receiver<BatteryState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, camera_interface, battery_state))
}

async fn handle_socket(
    mut socket: WebSocket, 
    camera_interface: watch::Receiver<CameraInterface>,
    battery_state: watch::Receiver<BatteryState>
) {
    let mut merged_stream = select_all([
        WatchStream::from_changes(camera_interface)
            .map(SharedMessage::CameraInterfaceUpdate)
            .boxed(),
        WatchStream::from_changes(battery_state)
            .map(SharedMessage::BatteryState)
            .boxed()
    ]);

    while let Some(message) = merged_stream.next().await {
        let Ok(json) = serde_json::to_string(&message) else {
            continue;
        };
        if socket.send(Message::Text(json.into())).await.is_err() {
            break;
        }
    }
}