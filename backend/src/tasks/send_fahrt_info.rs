use std::time::Duration;

use shared_data::FahrtInfo;
use tokio::select;

use crate::api::toggle_fahrt::Fahrt;


pub async fn send_fahrt_info_task(
    mut fahrt_receiver: tokio::sync::watch::Receiver<Option<Fahrt>>,
    fahrt_info_sender: tokio::sync::watch::Sender<Option<FahrtInfo>>,
) {
    let mut is_some = false;
    let mut timer = tokio::time::interval(Duration::from_secs(1));

    loop {
        select! {
            _ = timer.tick() => {
                let fahrt_info = fahrt_receiver.borrow().as_ref().map(|fahrt| fahrt.get_info());
                if fahrt_info.is_some() {
                    let _ = fahrt_info_sender.send(fahrt_info);
                }
            }
            _ = fahrt_receiver.changed() => {
                let fahrt_info = fahrt_receiver.borrow().as_ref().map(|fahrt| fahrt.get_info());
                if is_some != fahrt_info.is_some() {
                    is_some = fahrt_info.is_some();
                    let _ = fahrt_info_sender.send(fahrt_info);
                }
            }
        }
    }
}