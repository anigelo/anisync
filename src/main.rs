mod config;
mod sync;

extern crate dotenv;

use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let sync = warp::path!("sync")
        .map(|| {
            sync::run_sync();
            warp::http::StatusCode::OK
        });

    warp::serve(sync)
        .run(([0, 0, 0, 0], 8025))
        .await;
}

