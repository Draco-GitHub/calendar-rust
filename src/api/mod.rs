use chrono::Utc;
use warp::{Filter, Rejection, Reply};
use crate::api::auctions::auctions_routes;
use crate::api::bazaar::bazaar_routes;
use crate::api::calendar::calendar_routes;

mod bazaar;
mod auctions;
mod calendar;


pub fn build_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&serde_json::json!({
                "status": "healthy",
                "timestamp": Utc::now().timestamp()
            }))
        });

    let bazaar_routes = bazaar_routes();
    let auction_routes = auctions_routes();
    let calendar_routes = calendar_routes();

    health
        .or(bazaar_routes)
        .or(auction_routes)
        .or(calendar_routes)
        .with(warp::cors().allow_any_origin())
        .with(warp::log("api"))
}