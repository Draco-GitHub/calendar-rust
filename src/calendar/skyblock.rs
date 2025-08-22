use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::calendar::calendar::Calendar;
use crate::calendar::event::Event;
use crate::helpers::read_json_from_file;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkyblockDay {
    day: i8,
    month: i8,
    year: i16,
}
impl SkyblockDay {
    pub fn new( day: i8, month: i8, year: i16) -> Self {
        SkyblockDay { day, month, year }
    }
    pub fn as_datetime(&self) -> DateTime<Utc> {
        let total_days = ((self.year as i32 - 1) * 372) + ((self.month as i32 - 1) * 31) + (self.day as i32 -1);
        let total_real_minutes = total_days * 20;
        let real_duration = Duration::minutes(total_real_minutes.into());
        const YEAR_START_TIMESTAMP: i64 = 1560275700;
        let year_start = DateTime::from_timestamp(YEAR_START_TIMESTAMP, 0).unwrap();
        year_start + real_duration
    }

    pub fn date_to_skyblock(date: DateTime<Utc>) -> Self {
        const YEAR_START_TIMESTAMP: i64 = 1560275700;
        let time_delta_minutes = date.signed_duration_since(
            DateTime::from_timestamp(YEAR_START_TIMESTAMP, 0).unwrap()
        ).num_minutes();

        let delta_days = time_delta_minutes / 20;
        let delta_years = delta_days / 372;
        let year = 1 + delta_years as i32;
        let remaining_days = delta_days % 372;
        let month = (remaining_days / 31) as i32 + 1;
        let day = (remaining_days % 31) as i32 + 1;

        SkyblockDay::new(day as i8, month as i8, year as i16)
    }

    pub fn get_events(&mut self, previous_events: &Vec<Event>, previous_elections: &Vec<Election>) -> Vec<Event> {
        let mut events = Vec::new();

        for event in previous_events {
            if event.modulo(self.as_datetime()) == 0 {
                events.push(event.clone()) // Clone the event to own it
            }
        }

        if let Some(election) = Self::get_election(self.year+1, previous_elections) {
            let election_events = election.get_events();
            for event in election_events {
                if event.get_start_time() == self.as_datetime() {
                    events.push(event);
                }
            }
        }
        events
    }

    fn get_next_skyblock_day(date: DateTime<Utc>) -> DateTime<Utc> {
        let minute = date.minute();
        let valid_minutes = [15, 35, 55];
        for &valid_minute in &valid_minutes {
            if minute < valid_minute {
                return date.with_minute(valid_minute).unwrap().with_second(0).unwrap();
            }
        }
        date.checked_add_signed(Duration::hours(1)).unwrap().with_minute(15).unwrap().with_second(0).unwrap()
    }

    fn get_election(sb_year: i16, previous_elections: &Vec<Election>) -> Option<&Election> {
        for election in previous_elections {
            if election.year == sb_year {
                return Some(election);
            }
        }
        None
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Election {
    mayor: String,
    minister: String,
    perks: Vec<String>,
    year: i16,
    start: DateTime<Utc>,
    end: DateTime<Utc>
}

impl Election {

    async fn get_ongoing_election() -> Election {
        let response = reqwest::get("https://api.hypixel.net/v2/resources/skyblock/election")
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();

        let mayor = response["mayor"].clone();
        let mayor_name = mayor["name"].as_str().unwrap().to_string();

        let year = response["year"].as_i64().unwrap() as i16;
        let minister = mayor["minister"].clone();
        let minister_name = minister["name"].as_str().unwrap().to_string();

        let perks: Vec<String> = mayor["perks"].as_array().unwrap().iter()
            .chain(minister["perks"].as_array().unwrap().iter())
            .map(|perk| perk["name"].as_str().unwrap().to_string())
            .collect();

        let start: DateTime<Utc> = SkyblockDay::new(27,5,year).as_datetime();
        let end: DateTime<Utc> = SkyblockDay::date_to_skyblock(start + Duration::seconds(403200)).as_datetime();
        Election { mayor: mayor_name, minister: minister_name, perks, year, start, end }
    }

    fn get_events(&self) -> Vec<Event> {
        let mut events = Vec::new();

        let create_event = |name: &str, start: DateTime<Utc>, duration_hours: i64, interval: i64| {
            Event::new(
                name.to_string(),
                "".to_string(),
                self.start - Duration::minutes(3),
                start,
                start + Duration::hours(duration_hours),
                interval,
                0,
                120
            )
        };

        if self.perks.contains(&"Fishing Festival".to_string()) {
            events.extend((5..15).map(|i| {
                let start = SkyblockDay::new(1, i % 12, self.year).as_datetime();
                create_event("Fishing Festival", start, 1, 3600)
            }));
        } else if self.perks.contains(&"Mining Fiesta".to_string()) {
            events.extend((0..3).map(|i| {
                let start = SkyblockDay::new(1, i % 12, self.year).as_datetime();
                create_event("Mining Fiesta", start, 5, 18000)
            }));
        } else if self.perks.contains(&"Mythological Ritual".to_string()) {
            events.push(create_event("Mythological Ritual", self.start, 5, 446400));
        } else if self.perks.contains(&"Chivalrous Carnival".to_string()) {
            events.push(create_event("Chivalrous Carnival", self.start, 5, 446400));
        }

        events
    }
}

pub fn generate_calendar(from: DateTime<Utc>, to: DateTime<Utc>) -> Calendar {
    let mut calendar = Calendar::new("Skyblock".to_string(), None);
    let mut next_valid_day = SkyblockDay::get_next_skyblock_day(from);
    let previous_events:Vec<Event> = read_json_from_file("skyblock_events.json").unwrap();
    let previous_elections:Vec<Election> = read_json_from_file("elections.json").unwrap();

    while next_valid_day < to {
        let mut sb_day = SkyblockDay::date_to_skyblock(next_valid_day);
        let sb_events = sb_day.get_events(&previous_events, &previous_elections);
        for event in sb_events {
            calendar.add_event(event)
        }
        next_valid_day += Duration::minutes(20)
    }
    calendar
}