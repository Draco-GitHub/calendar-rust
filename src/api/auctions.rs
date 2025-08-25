use crate::api::auction_items::{
    Armor, ArmorType, AuctionItem, AuctionListing, HypixelAuction, Pet,
};
use chrono::Utc;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;
use warp::{Filter, Rejection, Reply};
static TRACKING: Mutex<Vec<String>> = Mutex::new(Vec::new());
pub fn auctions_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let auction_path = warp::path("auction");

    // GET /auction
    let list_auctions = auction_path
        .and(warp::get())
        .and(warp::path::end())
        .and_then(list_auctions_handler);

    // POST /auction/track
    let track_auction = auction_path
        .and(warp::path("track"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(track_auction_handler);

    // GET /auction/lowestbin?item=ITEM_NAME
    let lowest_bin = auction_path
        .and(warp::path("lowestbin"))
        .and(warp::get())
        .and(warp::query::<String>())
        .and_then(lowest_bin_handler);

    list_auctions.or(track_auction).or(lowest_bin)
}

async fn list_auctions_handler() -> Result<impl Reply, Rejection> {
    let x: HypixelAuction = reqwest::get("https://api.hypixel.net/v2/skyblock/auctions")
        .await
        .unwrap()
        .json::<HypixelAuction>()
        .await
        .unwrap();
    println!("{:?}", x.auctions);

    Ok(warp::reply::json(
        &serde_json::json!({"test": format!("{:?}",x.auctions)}),
    ))
}

async fn track_auction_handler(item: String) -> Result<impl Reply, Rejection> {
    TRACKING.lock().unwrap().push(item.clone());
    let response = serde_json::json!({
        "message": format!("Now tracking auctions for '{}'", item),
        "item": item,
        "tracked_since": Utc::now()
    });

    Ok(warp::reply::json(&response))
}

async fn lowest_bin_handler(item: String) -> Result<impl Reply, Rejection> {
    let lowest_bins = HashMap::from([
        ("Dragon Sword".to_string(), 75000.0),
        ("Terminator".to_string(), 30000.0),
        ("Armor".to_string(), 45000.0),
    ]);

    match lowest_bins.get(&item) {
        Some(price) => Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({
                "item": item,
                "lowest_bin": price,
                "last_updated": Utc::now()
            })),
            warp::http::StatusCode::OK,
        )),
        None => Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({
                "error": format!("No BIN data found for '{}'", item)
            })),
            warp::http::StatusCode::NOT_FOUND,
        )),
    }
}
