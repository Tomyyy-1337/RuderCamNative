use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{StreamExt, stream::select_all};
use shared_data::{BatteryState, CameraInterface, FinishedFahrt, Message as SharedMessage};
use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    camera_interface: watch::Receiver<CameraInterface>,
    battery_receiver: watch::Receiver<BatteryState>,
    gps_satelite_count: watch::Receiver<u8>,
    finished_fahrt: watch::Receiver<FinishedFahrt>,
    current_stats: watch::Receiver<shared_data::CurrentStats>,
    fahrt_info: watch::Receiver<Option<shared_data::FahrtInfo>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, 
        camera_interface, 
        battery_receiver,
        gps_satelite_count, 
        finished_fahrt,
        current_stats,
        fahrt_info
    ))
}

async fn handle_socket(
    mut socket: WebSocket, 
    camera_interface: watch::Receiver<CameraInterface>,
    battery_receiver: watch::Receiver<BatteryState>,
    gps_satelite_count: watch::Receiver<u8>,
    mut finished_fahrt: watch::Receiver<FinishedFahrt>,
    current_stats: watch::Receiver<shared_data::CurrentStats>,
    fahrt_info: watch::Receiver<Option<shared_data::FahrtInfo>>,
) {
    finished_fahrt.mark_unchanged();

    let mut merged_stream = select_all([
        WatchStream::from_changes(camera_interface)
            .map(SharedMessage::CameraInterfaceUpdate)
            .boxed(),
        WatchStream::from_changes(battery_receiver)
            .map(SharedMessage::BatteryState)
            .boxed(),
        WatchStream::from_changes(gps_satelite_count)
            .map(SharedMessage::GpsSateliteCount)
            .boxed(),
        WatchStream::from_changes(finished_fahrt)
            .map(SharedMessage::FinishedFahrt)
            .boxed(),
        WatchStream::from_changes(current_stats)
            .map(SharedMessage::CurrentStats)
            .boxed(),
        WatchStream::from_changes(fahrt_info)
            .map(SharedMessage::FahrtInfoUpdate)
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