use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use chrono::{Date, DateTime, Duration, FixedOffset, Local, Utc};
use serde::{Deserialize, Serialize};
use serde::de::{DeserializeOwned, Error};
use uuid::Uuid;
use crate::skyblock::get_skyblock_events;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Event {
    id: Uuid,
    title: String,
    description: Option<String>,
    notify_at: DateTime<Utc>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    duration: i64,
    recurrence: Option<i64>
}

impl Event {
    fn new(title: &str, description:Option<String>, notify_at:DateTime<Utc>, start_time: DateTime<Utc>, end_time: DateTime<Utc>, duration:i64, recurrence:Option<i64>) -> Self {
        Event { id: Uuid::new_v4(), title: title.to_string(), description, notify_at, start_time, end_time, duration, recurrence }
    }

    fn modulo_is_zero(&self, date:DateTime<Utc>) -> bool {
        if (self.start_time.signed_duration_since(date).num_seconds() % self.recurrence.unwrap()) == 0 {
            return true
        }
        false
    }

    fn is_upcoming(&self, date: DateTime<Utc>) -> bool {
        self.start_time > date
    }

    fn next_occurrence(&self, date:DateTime<Utc>) -> Option<DateTime<Utc>> {
        if self.recurrence.is_some() {
            let mut occurrence_start = self.start_time;
            while occurrence_start <= date {
                occurrence_start = occurrence_start + Duration::seconds(self.recurrence?);
            }
            return Some(occurrence_start)
        }
        None
    }
}
struct User {
    id:Uuid,
    name: String,
    events: HashMap<Uuid, Event>
}
impl User {
    fn new(name:String) -> Self {
        User { id: Uuid::new_v4(), name, events: HashMap::new() }
    }

    fn add_event(&mut self, event: Event) {
        self.events.insert(event.id, event);
    }

    fn get_event(&self, event_id: Uuid) -> Option<&Event> {
        self.events.get(&event_id)
    }

    fn list_events(&self) -> Vec<&Event> {
        self.events.values().collect()
    }

    fn find_events_at(&self, time: DateTime<Utc>) -> Vec<&Event> {
        self.events.values().filter(|event| event.is_happening_at(time)).collect()
    }

    fn find_upcoming_events(&self, date: DateTime<Utc>) -> Vec<&Event> {
        let mut upcoming_events: Vec<&Event> = self.events.values()
            .filter(|event| event.is_upcoming(date))
            .collect();

        upcoming_events.sort_by_key(|event| event.start_time);
        upcoming_events
    }

}

struct CalendarDataBase {
    users: HashMap<Uuid, User>
}
impl CalendarDataBase {
    fn new() -> Self {
        CalendarDataBase {users: HashMap::new()}
    }
    fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
    fn get_user(&self, user_id:Uuid) -> Option<&User> {
        self.users.get(&user_id)
    }
    fn list_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }
}



