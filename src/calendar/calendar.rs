use std::collections::HashMap;
use std::fmt;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
pub(crate) use crate::calendar::event::Event;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calendar {
    id: Uuid,
    title: String,
    description: String,
    events: HashMap<Uuid, Event>,
}
impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let events_list = self.list_events()
            .iter()
            .map(|event| format!("{}", event))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "id: {} title: {} description: {} events: [{}]", self.id, self.title, self.description, events_list)
    }
}

impl Calendar {
    pub fn new(title: String, description: Option<String>) -> Self {
        if description.is_some() {
            return Calendar { id: Uuid::new_v4(), title, description:description.unwrap(), events: HashMap::new() }
        }
        Calendar {id: Uuid::new_v4(), title, description:"".to_string(), events: HashMap::new() }


    }
    pub fn get_id(&self) -> &Uuid { &self.id }
    pub fn add_event(&mut self, event: Event) {
        self.events.insert(Uuid::new_v4(), event);
    }
    pub fn get_event(&self, event_id: Uuid) -> Option<&Event> {
        self.events.get(&event_id)
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn list_events(&self) -> Vec<&Event> {
        self.events.values().collect()
    }
    pub fn find_upcoming_events(&self, date: DateTime<Utc>) -> Vec<&Event> {
        let mut upcoming_events: Vec<&Event> = self.events.values()
            .filter(|event| event.is_upcoming(date))
            .collect();

        upcoming_events.sort_by_key(|event| event.start_time);
        upcoming_events
    }
}