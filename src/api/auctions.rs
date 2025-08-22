use std::collections::HashMap;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum AuctionItem {
    Pet(Pet),
    Armor(Armor),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Pet {
    name: String,
    level: i16,
    exp: i32,
    candy: i8,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
enum ArmorType {
    Helmet,
    Chestplate,
    Leggings,
    Boots,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Armor {
    name: String,
    armor_type: ArmorType,
    enchants: Vec<String>,
    gemstones: Vec<String>,
    fuming: i16,
    art_of_piece: i16,
    reforge: String,
    star: i16,
}

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

    list_auctions
        .or(track_auction)
        .or(lowest_bin)
}


async fn list_auctions_handler() -> Result<impl Reply, Rejection> {
    let sample_auctions: Vec<AuctionItem> = vec![
        AuctionItem::Pet(Pet {
            name: "Golden Dragon".to_string(),
            level: 200,
            exp: 100_000_000,
            candy: 0,
        }),
        AuctionItem::Armor(Armor {
            name: "Necron's Helmet".to_string(),
            armor_type: ArmorType::Helmet,
            enchants: vec!["Protection V".to_string(), "Growth VI".to_string()],
            gemstones: vec!["Jasper".to_string()],
            fuming: 5,
            art_of_piece: 1,
            reforge: "Ancient".to_string(),
            star: 5,
        }),
    ];


    Ok(warp::reply::json(&sample_auctions))
}

async fn track_auction_handler(item: String) -> Result<impl Reply, Rejection> {
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
        ("Magic Bow".to_string(), 30000.0),
        ("Enchanted Armor".to_string(), 45000.0),
    ]);

    match lowest_bins.get(&item) {
        Some(price) => Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({
                "item": item,
                "lowest_bin": price,
                "last_updated": Utc::now()
            })),
            warp::http::StatusCode::OK
        )),
        None => Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({
                "error": format!("No BIN data found for '{}'", item)
            })),
            warp::http::StatusCode::NOT_FOUND
        )),
    }
}
