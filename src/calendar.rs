use std::collections::HashMap;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde::de::{Error};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    title: String,
    description: Option<String>,
    notify_at: DateTime<Utc>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    duration: i64,
    recurrence: Option<i64>,
    remind: Option<i64>
}


impl Event {
    pub fn new(title: &str, description:Option<String>, notify_at:DateTime<Utc>, start_time: DateTime<Utc>, end_time: DateTime<Utc>, duration:i64, recurrence:Option<i64>, remind:Option<i64>) -> Self {
        Event { title: title.to_string(), description, notify_at, start_time, end_time, duration, recurrence, remind}
    }
    pub fn modulo(&self, date:DateTime<Utc>) -> i64 { self.start_time.signed_duration_since(date).num_seconds() % self.recurrence.unwrap() }
    fn is_upcoming(&self, date: DateTime<Utc>) -> bool {
        self.start_time > date
    }
    pub(crate) fn get_start_time(&self) -> DateTime<Utc> {
        self.start_time
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calendar {
    id: Uuid,
    title: String,
    description: Option<String>,
    events: HashMap<Uuid, Event>,
}
impl Calendar {
    pub fn new(title: String, description: Option<String>) -> Self {
        Calendar { id: Uuid::new_v4(), title, description, events: HashMap::new() }
    }

    pub fn get_id(&self) -> Uuid { self.id.clone() }
    pub fn add_event(&mut self, event: Event) {
        self.events.insert(Uuid::new_v4(), event);
    }

    pub fn get_event(&self, event_id: Uuid) -> Option<&Event> {
        self.events.get(&event_id)
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id:Uuid,
    name: String,
    calendars: HashMap<Uuid, Calendar>,
}
impl User {
    pub fn new(name:String) -> Self {
        User { id: Uuid::new_v4(), name, calendars: HashMap::new() }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_calendar(&mut self, calendar: Calendar) {
        self.calendars.insert(Uuid::new_v4(), calendar);
    }

    pub fn get_calendar(&self, id: Uuid) -> Option<&Calendar> {
        self.calendars.get(&id)
    }

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataBase {
    users: HashMap<Uuid, User>
}
impl DataBase {
    pub fn new() -> Self {
        DataBase {users: HashMap::new()}
    }
    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
    pub fn get_user(&self, user_id:Uuid) -> Option<&User> {
        self.users.get(&user_id)
    }
    pub fn list_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }
}



