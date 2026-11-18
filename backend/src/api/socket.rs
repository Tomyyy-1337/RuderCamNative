// use axum::{
//     extract::{
//         State,
//         ws::{Message, WebSocket, WebSocketUpgrade},
//     },
//     response::IntoResponse,
// };
// use shared_data::{CameraInterface, Message as SharedMessage};
// use tokio::sync::watch;

// pub async fn ws_handler(
//     ws: WebSocketUpgrade,
//     State(receiver): State<watch::Receiver<CameraInterface>>,
// ) -> impl IntoResponse {
//     ws.on_upgrade(move |socket| handle_socket(socket, receiver))
// }

// async fn handle_socket(mut socket: WebSocket, mut receiver: watch::Receiver<CameraInterface>) {
//     loop {
//         tokio::select! {
//             changed = receiver.changed() => {
//                 if changed.is_err() {
//                     break;
//                 }
//                 let settings = *receiver.borrow_and_update();
//                 let Ok(json) = serde_json::to_string(&SharedMessage::CameraInterfaceUpdate(settings)) else {
//                     continue;
//                 };
//                 if socket.send(Message::Text(json.into())).await.is_err() {
//                     break;
//                 }
//             }
//             incoming = socket.recv() => {
//                 match incoming {
//                     Some(Ok(Message::Close(_))) | Some(Err(_)) | None => break,
//                     _ => {}
//                 }
//             }
//         }
//     }
// }
