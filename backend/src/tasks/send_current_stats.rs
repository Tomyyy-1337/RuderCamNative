use std::time::Duration;
use shared_data::CurrentStats;
use tokio::{sync::watch, time::MissedTickBehavior};

/// Read current bpm and velocity(TODO) and update the current stats sender once per second.
pub async fn send_current_stats_task(
    bpm_receiver: watch::Receiver<f32>,
    velocity_receiver: watch::Receiver<f32>,
    current_stats_sender: watch::Sender<CurrentStats>,
) {
    let mut timer = tokio::time::interval(Duration::from_secs(1));
    timer.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        timer.tick().await;

        let bpm = *bpm_receiver.borrow();
        let velocity = *velocity_receiver.borrow();

        let current_states = CurrentStats { bpm, velocity };
        let _ = current_stats_sender.send_if_modified(|current| {
            let changed = *current != current_states;
            if changed {
                *current = current_states;
            }
            changed
        });
    }
}