use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::calendar::{Calendar, Event};
use crate::helpers::read_json_from_file;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkyblockDay {
    day: i32,
    month: i32,
    year: i32,
}
impl SkyblockDay {
    pub fn new( day: i32, month: i32, year: i32) -> Self {
        SkyblockDay { day, month, year }
    }
    pub(crate) fn as_datetime(&self) -> DateTime<Utc> {
        let year_start = DateTime::parse_from_str("2019-06-11 17:55:00+00:00", "%Y-%m-%d %H:%M:%S%z")
            .expect("Failed to parse DateTime")
            .with_timezone(&Utc);
        let total_days = ((self.year - 1) * 12 * 31) + ((self.month - 1) * 31) + (self.day - 1);
        let total_real_minutes = total_days * 20;
        let real_duration = Duration::minutes(total_real_minutes.into());
        year_start + real_duration
    }

    pub(crate) fn date_to_skyblock(date:DateTime<Utc>) -> Self {
        let year_start = DateTime::parse_from_str("2019-06-11 17:55:00+00:00", "%Y-%m-%d %H:%M:%S%z")
            .expect("Failed to parse DateTime")
            .with_timezone(&Utc);
        let time_delta_minutes = date.signed_duration_since(year_start).num_minutes();
        let delta_days = time_delta_minutes as f32 / 20.0;
        let delta_years = (delta_days / 372.0).floor();
        let year = 1 + delta_years as i32;
        let remaining_days = delta_days - (delta_years * 372.0);
        let month = (remaining_days / 31.0).floor() as i32 + 1;
        let day = (remaining_days % 31.0).floor() as i32 + 1;
        SkyblockDay::new(day, month, year)
    }

    pub(crate) fn get_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        let election: Election = Self::get_election(self.year).expect("Failed to get election");
        let election_events = election.get_events();
        println!("{:?}", election);
        for event in election_events {
            if event.get_start_time() == self.as_datetime() {
                events.push(event);
            }
        }
        let skyblock_events:Vec<Event> = read_json_from_file("skyblock.json").expect("Failed to read skyblock.json");
        for event in skyblock_events {
            if event.modulo(self.as_datetime()) == 0 {
                println!("{event:?}")
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

    fn get_election(sb_year: i32) -> Option<Election> {
        let elections:Vec<Election> = read_json_from_file("elections.json").expect("Failed to read json");
        for election in elections {
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
    year: i32,
    start: DateTime<Utc>,
    end: DateTime<Utc>
}

impl Election {
    pub fn new(mayor: String, minister: String, perks: Vec<String>, year: i32, start:DateTime<Utc>, end:DateTime<Utc>) -> Election {
        Election { mayor, minister, perks, year, start, end }
    }
    async fn get_ongoing_election() -> Result<Election, reqwest::Error> {
        // Make the API request and deserialize the response into a JSON `Value`
        let request = reqwest::get("https://api.hypixel.net/v2/resources/skyblock/election").await?.json::<Value>().await?;

        // Extract the mayor object and its fields
        let mayor = request.get("mayor").expect("Failure to get mayor value");
        let mayor_name = mayor.get("name").expect("Failure to get mayor name value").as_str().expect("Expected a string").to_string();
        let mayor_perks = mayor.get("perks").expect("Failure to get mayor perks value");

        // Extract the year as an i32
        let year = request.get("year").expect("Failure to get year value").as_i64().expect("Expected a number") as i32;

        // Extract the minister object and its fields
        let minister = mayor.get("minister").expect("Failure to get minister value");
        let minister_name = minister.get("name").expect("Failure to get minister name value").as_str().expect("Expected a string").to_string();
        let minister_perks = minister.get("perks").expect("Failure to get minister perks value");

        // Collect perks for both the mayor and minister
        let mut perks = Vec::new();
        for perk in mayor_perks.as_array().expect("Expected perks array") {
            perks.push(perk.get("name").expect("Failure to get perk name").as_str().expect("Expected a string").to_string());
        }
        for perk in minister_perks.as_array().expect("Expected perks array") {
            perks.push(perk.get("name").expect("Failure to get perk name").as_str().expect("Expected a string").to_string());
        }

        // Calculate start and end dates using your custom calendar system
        let start: DateTime<Utc> = SkyblockDay::new(27,5,year).as_datetime();
        let end: DateTime<Utc> = SkyblockDay::date_to_skyblock(start + Duration::seconds(403200)).as_datetime();

        // Return the Election object wrapped in Ok to indicate success
        Ok(Election::new(mayor_name, minister_name, perks, year, start, end))
    }
    fn get_events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        if self.perks.contains(&"Fishing Festival".to_string()) {
            for i in 5..15 {
                let start = SkyblockDay::new(1,i%12, self.year).as_datetime();
                events.push(Event::new("Fishing Festival", Some("".to_string()), Some(self.start - Duration::minutes(3)), start, start+Duration::hours(1), 3600, None, Some(120)));
            }
        }
        else if self.perks.contains(&"Mining Fiesta".to_string()) {
            for i in 0..3 {
                let start = SkyblockDay::new(1,i%12, self.year).as_datetime();
                events.push(Event::new("Mining Fiesta", Some("".to_string()), Some(self.start - Duration::minutes(3)), start, start+Duration::hours(5), 18000, None, Some(120)));
            }
        }
        else if self.perks.contains(&"Mythological Ritual".to_string()) {
            events.push(Event::new("Mining Fiesta", Some("".to_string()), Some(self.start - Duration::minutes(3)), self.start, self.start+Duration::hours(5), 446400, None, Some(120)));
        }
        else if self.perks.contains(&"Chivalrous Carnival".to_string()) {
            events.push(Event::new("Chivalrous Carnival", Some("".to_string()), Some(self.start - Duration::minutes(3)), self.start, self.start+Duration::hours(5), 446400, None, Some(120)));
        }
        events
    }
}

pub fn generate_calendar(from: DateTime<Utc>, to: DateTime<Utc>) -> Calendar {
    let mut calendar = Calendar::new("Skyblock".to_string(), None);
    let mut next_valid_day = SkyblockDay::get_next_skyblock_day(from);
    let events:Vec<Event> = read_json_from_file("skyblock_events.json").unwrap();
    while next_valid_day < to {
        for event in &events {
            println!("{}", event.modulo(next_valid_day));
            if (event.modulo(next_valid_day) == 0 ) {
                let mut sb_day = SkyblockDay::date_to_skyblock(next_valid_day);
                let sb_events = sb_day.get_events();
                for sb_event in sb_events{
                    println!("Added event: {sb_event:?}");
                    calendar.add_event(sb_event)
                }
            }
        }
    }
    calendar
}





