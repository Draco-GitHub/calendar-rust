mod helpers;
mod calendar;
mod api;
mod logger;

use log::info;
use crate::logger::init_logger;

#[tokio::main]
async fn main() {
    init_logger();
    let api = api::build_routes();

    info!("API Server starting on http://localhost:7878");
    warp::serve(api)
        .run(([127, 0, 0, 1], 7878))
        .await;
}

fn seconds_to_dhm(seconds: i64) -> (i64, i64, i64) {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    (days, hours, minutes)
}
