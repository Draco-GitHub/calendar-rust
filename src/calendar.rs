use std::fs::File;
use chrono::{DateTime, Duration, FixedOffset, Local};
use serde::{Deserialize, Serialize};
use crate::skyblock::get_skyblock_events;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) notify_at: DateTime<FixedOffset>,
    pub(crate) start_time: DateTime<FixedOffset>,
    pub(crate) end_time: DateTime<FixedOffset>,
    pub(crate) duration: i32,
    pub(crate) repeat: bool,
    pub(crate) interval: i32,
    pub(crate) reminder: i8
}

pub fn get_events(path: &str) -> Vec<Event> {
    let file = File::open(path).unwrap();
    let events: Vec<Event> = serde_json::from_reader(file).unwrap();
    events
}

pub fn save_events(events: &Vec<Event>, path: &str) {
    let file = File::create(path).unwrap();
    serde_json::to_writer(file, events).unwrap();
}

pub fn get_calendar(start: DateTime<FixedOffset>, end:DateTime<FixedOffset>) -> Vec<Event> {
    let mut calendar = Vec::new();
    calendar.append(get_skyblock_events(start, end).as_mut());
    let events = get_events("events.json");

    for event in events {
        let delta_seconds = (start.signed_duration_since(event.start_time).num_seconds()/event.interval as i64) * event.interval as i64;
        let mut current_date = event.start_time + Duration::seconds(delta_seconds);
        while current_date < end {
            current_date += Duration::seconds(event.interval as i64);
            let mut new_event = event.clone();
            new_event.notify_at = current_date - Duration::seconds(event.reminder as i64);
            new_event.start_time = current_date;
            new_event.end_time = current_date + Duration::seconds(new_event.duration as i64);
            calendar.push(new_event)
        }
    }
    calendar.sort_by_key(|n| n.start_time);
    calendar
}