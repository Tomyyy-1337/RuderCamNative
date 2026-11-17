use tokio::{runtime::LocalOptions, task};

use crate::api::axum_server;

mod api;

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .max_blocking_threads(4)
        .enable_all()
        .build_local(LocalOptions::default())
        .unwrap()
        .block_on(async {
            task::spawn_local(axum_server::start_axum_server());

            futures::future::pending::<()>().await;
        });
}
