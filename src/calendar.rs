use std::fs::File;
use chrono::{DateTime, Duration, FixedOffset, Local};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    name: String,
    description: String,
    notify_at: DateTime<FixedOffset>,
    start_time: DateTime<FixedOffset>,
    end_time: DateTime<FixedOffset>,
    repeat: bool,
    interval: i32,
    reminder: i8
}

fn get_events() -> Vec<Event> {
    let file = File::open("events.json").unwrap();
    let events: Vec<Event> = serde_json::from_reader(file).unwrap();
    events
}

fn save_events(events: &Vec<Event>) {
    let file = File::create("events.json").unwrap();
    serde_json::to_writer(file, events).unwrap();
}

pub fn get_calendar(start: DateTime<FixedOffset>, end:DateTime<FixedOffset>) -> Vec<Event> {
    let mut calendar = Vec::new();
    let events = get_events();

    for event in events {
        let delta_seconds = (start.signed_duration_since(event.start_time).num_seconds()/event.interval as i64) * event.interval as i64;
        let mut current_date = event.start_time + Duration::seconds(delta_seconds);
        while current_date < end {
            current_date += Duration::seconds(event.interval as i64);
            let mut new_event = event.clone();
            new_event.notify_at = current_date - Duration::seconds(event.reminder as i64);
            new_event.start_time = current_date;
            new_event.end_time = current_date + Duration::seconds(event.interval as i64);
            calendar.push(new_event)
        }
    }
    calendar.sort_by_key(|n| n.start_time);
    calendar
}