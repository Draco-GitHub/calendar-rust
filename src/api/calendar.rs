use chrono::Utc;
use warp::{Filter, Rejection, Reply};
use crate::calendar::calendar::Event;

pub fn calendar_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let calendar_path = warp::path("calendar");

    // GET /calendar/upcoming
    let upcoming = calendar_path
        .and(warp::path("upcoming"))
        .and(warp::get())
        .and_then(upcoming_events_handler);

    // GET /calendar/events
    let events = calendar_path
        .and(warp::path("events"))
        .and(warp::get())
        .and_then(all_events_handler);


    upcoming
        .or(events)
}

async fn upcoming_events_handler() -> Result<impl Reply, Rejection> {
    let now = Utc::now();
    let upcoming_events = vec![
        Event::new(
            "event name".to_string(),
            "".to_string(),
            now,
            now,
            now,
            10i64,
            10i64,
            10i64            
        )
    ];

    Ok(warp::reply::json(&upcoming_events))
}

async fn all_events_handler() -> Result<impl Reply, Rejection> {
    let now = Utc::now();
    let events = vec![
        Event::new(
            "event name".to_string(),
            "".to_string(),
            now,
            now,
            now,
            10i64,
            10i64,
            10i64
        )
    ];

    Ok(warp::reply::json(&events))
}
