use std::time::Duration;
use shared_data::BatteryState;
use tokio::{sync::watch, time::MissedTickBehavior};

/// Read battery state from the I2C interface and update the sender periodically.
pub async fn read_battery_task(battery_sender: watch::Sender<BatteryState>) {
    let mut timer = tokio::time::interval(Duration::from_secs(15));
    timer.set_missed_tick_behavior(MissedTickBehavior::Delay);
    
    #[cfg(target_os = "linux")]
    loop {
        timer.tick().await;
        let battery_percentage = crate::I2C_INTERFACE.read_battery_percentage().await;
        let _ = battery_sender.send(BatteryState { percentage: battery_percentage });
        
        if battery_percentage <= 2 {
            println!("Battery critically low ({}%), shutting down...", battery_percentage);
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            crate::hardware_interface::pi_interface::shutdown();
        } 
    }

    #[cfg(not(target_os = "linux"))]
    for i in 0.. {
        timer.tick().await;
        let _ = battery_sender.send(BatteryState { percentage: i*10 % 101 });
    }
}
