use std::fmt;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    title: String,
    description: String,
    notify_at: DateTime<Utc>,
    pub(crate) start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    duration: i64,
    recurrence: i64,
    remind: i64
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} - {})", self.title, self.start_time, self.end_time)
    }
}

impl Event {
    pub fn new(title: String, description:String, notify_at:DateTime<Utc>, start_time: DateTime<Utc>, end_time: DateTime<Utc>, duration:i64, recurrence:i64, remind:i64) -> Self {
        Event { title, description, notify_at, start_time, end_time, duration, recurrence, remind}
    }

    pub fn modulo(&self, date:DateTime<Utc>) -> i64 {
        self.start_time.signed_duration_since(date).num_seconds() % self.recurrence
    }

    pub(crate) fn is_upcoming(&self, date: DateTime<Utc>) -> bool {
        self.start_time > date
    }

    pub(crate) fn get_start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    fn next_occurrence(&self, date:DateTime<Utc>) -> Option<DateTime<Utc>> {
        if self.recurrence != 0 {
            let mut occurrence_start = self.start_time;
            while occurrence_start <= date {
                occurrence_start = occurrence_start + Duration::seconds(self.recurrence);
            }
            return Some(occurrence_start)
        }
        None
    }
}