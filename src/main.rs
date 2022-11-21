mod config;
mod sync;
mod fs_store;
mod api;

extern crate dotenv;

use std::time::Duration;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tokio::spawn(auto_sync());

    warp::serve(api::routes())
        .run(([0, 0, 0, 0], 8025))
        .await;
}

async fn auto_sync() {
    loop {
        let settings = config::Config::read();
        let scan_interval = Duration::from_secs(settings.scan_interval.into());
        let join_handle = tokio::task::spawn_blocking(|| sync::run(settings));
        if let Err(e) = join_handle.await {
            eprintln!("{:?}", e);
        }
        tokio::time::sleep(scan_interval).await;
    }
}

