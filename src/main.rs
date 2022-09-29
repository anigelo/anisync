mod config;
mod sync;

extern crate dotenv;

use std::time::Duration;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tokio::spawn(auto_sync());

    let sync = warp::path!("sync")
        .map(|| {
            sync::run_sync();
            warp::http::StatusCode::OK
        });

    warp::serve(sync)
        .run(([0, 0, 0, 0], 8025))
        .await;
}

async fn auto_sync() {
    loop {
        let join_handle = tokio::task::spawn_blocking(|| sync::run_sync());
        if let Err(e) = join_handle.await {
            eprintln!("{:?}", e);
        }
        tokio::time::sleep(Duration::from_secs(86400)).await;
    }
}

