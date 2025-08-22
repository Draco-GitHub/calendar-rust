use chrono::Utc;
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

pub fn bazaar_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let bazaar_path = warp::path("bazaar");

    // GET /bazaar
    let list_bazaar = bazaar_path
        .and(warp::get())
        .and(warp::path::end())
        .and_then(list_bazaar_handler);

    // POST /bazaar/track
    let track_item = bazaar_path
        .and(warp::path("track"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(track_bazaar_item_handler);

    list_bazaar.or(track_item)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BazaarItem {
    name: String,
    sell_price: f64,
    sell_volume: i32,
    sell_moving_week: i32,
    sell_orders: i32,
    buy_price: f64,
    buy_volume: i32,
    buy_moving_week: i32,
    buy_orders: i32,
}

async fn list_bazaar_handler() -> Result<impl Reply, Rejection> {
    let sample_items = vec![
        BazaarItem {
            name: "CORRUPTED_BAIT".to_string(),
            sell_price: 2.18318238828381,
            sell_volume: 533299,
            sell_moving_week: 266680,
            sell_orders: 24,
            buy_price: 908.4,
            buy_volume: 16754,
            buy_moving_week: 370265,
            buy_orders: 18
        }
    ];
    Ok(warp::reply::json(&sample_items))
}

async fn track_bazaar_item_handler(item: String) -> Result<impl Reply, Rejection> {
    let response = serde_json::json!({
        "message": format!("Now tracking '{}' in bazaar", item),
        "tracked_since": Utc::now()
    });
    Ok(warp::reply::json(&response))
}