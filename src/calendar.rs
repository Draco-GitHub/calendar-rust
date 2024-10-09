use std::fs::File;
use serde::{Deserialize, Serialize};
use crate::skyblock::SkyblockEvent;


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Event {
    name: String,
    description: String,
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

