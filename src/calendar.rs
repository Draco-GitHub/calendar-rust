use std::fs::File;
use chrono::{DateTime, Duration, FixedOffset, Local};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    name: String,
    description: String,
    notify_at: String,
    start_time: String,
    end_time: String,
    repeat: bool,
    interval: i32,
    reminder: i32
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

//returns a list of events from the start date to the end date
pub fn get_calendar(start: DateTime<FixedOffset>, end:DateTime<FixedOffset>) -> Vec<Event> {
    println!("running");
    let mut calendar = Vec::new();
    let events = get_events();
    let time_interval = start.signed_duration_since(end).num_seconds();

    // println!("{delta_seconds}");


    for event in events {
        let last_event_start_date = DateTime::parse_from_rfc3339(&*event.start_time).expect("Failed to parse time");
        let delta_seconds = (start.signed_duration_since(last_event_start_date).num_seconds()/event.interval as i64) * event.interval as i64;
        let start_iter_date = last_event_start_date + Duration::seconds(delta_seconds);

        let mut current_date = start_iter_date;
        while current_date < end {
            current_date += Duration::seconds(event.interval as i64);

            let mut new_event = event.clone();
            new_event.notify_at = (current_date - Duration::seconds(event.reminder as i64)).to_rfc2822();
            new_event.start_time = current_date.to_rfc2822();
            new_event.end_time = (current_date + Duration::seconds(event.interval as i64)).to_rfc2822();
            calendar.push(new_event)
        }
    }
    calendar
}