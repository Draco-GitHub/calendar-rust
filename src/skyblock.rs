use chrono::{DateTime, Duration, Timelike, Utc};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::calendar::Event;
use crate::helpers::read_json_from_file;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SkyblockDay {
    day: i32,
    month: i32,
    year: i32,
    events: Vec<String>
}
impl SkyblockDay {
    pub fn new( day: i32, month: i32, year: i32) -> Self {
        let events:Vec<String> = Vec::new();
        SkyblockDay { day, month, year, events }
    }

    fn convert_to_date(day:i32, month:i32, year:i32) -> DateTime<Utc> {
        let year_start: DateTime<Utc> = "2024-09-30 05:55:00".parse().expect("Failed to parse datetime");
        let total_days = ((year - 376) * 12 * 31) + ((month - 1) * 31) + (day - 1);
        let total_real_minutes = total_days * 20;
        let real_duration = Duration::minutes(total_real_minutes.into());
        year_start + real_duration
    }

    fn date_to_skyblock(date:DateTime<Utc>) -> Self {
        let year_start: DateTime<Utc> = "2024-09-30 05:55:00".parse().expect("Failed to parse datetime");
        let time_delta_minutes = date.signed_duration_since(year_start).num_minutes();
        let delta_days = time_delta_minutes as f32 / 20.0;
        let delta_years = (delta_days / 372.0).floor();
        let year = 376 + delta_years as i32;
        let remaining_days = delta_days - (delta_years * 372.0);
        let month = (remaining_days / 31.0).floor() as i32 + 1;
        let day = (remaining_days % 31.0).floor() as i32 + 1;
        SkyblockDay::new(day, month, year)
    }

    fn get_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        let election: Election = Self::get_election(self.year).expect("Failed to get election");
        let election_events = election.get_events();
        for event in election_events {
            if event.get_start_time() == Self::convert_to_date(self.day, self.month, self.year) {
                events.push(event);
            }
        }
        let skyblock_events:Vec<Event> = read_json_from_file("skyblock.json").expect("Failed to read skyblock.json");
        for event in skyblock_events {
            if event.modulo(SkyblockDay::convert_to_date(self.day, self.month, self.year)) == 0 {
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
        let elections:Vec<Election> = read_json_from_file("election.json").expect("Failed to read json");
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
        let start: DateTime<Utc> = SkyblockDay::convert_to_date(27, 5, year);
        let end: DateTime<Utc> = SkyblockDay::convert_to_date(25, 5, year + 1); // Adjust dates as needed

        // Return the Election object wrapped in Ok to indicate success
        Ok(Election::new(mayor_name, minister_name, perks, year, start, end))
    }



    fn get_events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        if self.perks.contains(&"Fishing Festival".to_string()) {
            for i in 5..15 {
                let start = SkyblockDay::convert_to_date(1,i%12, self.year);
                events.push(Event::new("Fishing Festival", Some("".to_string()), start-Duration::minutes(3), start, start+Duration::hours(1), 3600, None));
            }
        }
        else if self.perks.contains(&"Mining Fiesta".to_string()) {
            for i in 0..3 {
                let start = SkyblockDay::convert_to_date(1,i%12, self.year);
                events.push(Event::new("Mining Fiesta", Some("".to_string()), start-Duration::minutes(3), start, start+Duration::hours(5), 18000, None));
            }
        }
        else if self.perks.contains(&"Mythological Ritual".to_string()) {
            events.push(Event::new("Mining Fiesta", Some("".to_string()), self.start - Duration::minutes(3), self.start, self.start+Duration::hours(5), 446400, None));
        }
        else if self.perks.contains(&"Chivalrous Carnival".to_string()) {
            events.push(Event::new("Chivalrous Carnival", Some("".to_string()), self.start - Duration::minutes(3), self.start, self.start+Duration::hours(5), 446400, None));
        }
        events
    }
}

// pub fn get_skyblock_events(start: DateTime<Utc>, end:DateTime<Utc>) -> Vec<Event> {
//     let mut calendar = Vec::new();
//     let mut events:Vec<Event> = read_json_from_file("skyblock_events.json").unwrap();
//     let mut next_valid_skyblock_day = get_next_skyblock_day(start).unwrap();
//
//     while next_valid_skyblock_day < end {
//         let skyblock_date = SkyblockDay::date_to_skyblock(next_valid_skyblock_day);
//         let election = get_election(skyblock_date.year).unwrap();
//
//     }
//
//     while next_valid_skyblock_day < end {
//         let skyblock_date = datetime_to_skyblock(next_valid_skyblock_day);
//         let election = get_election(skyblock_date.2).unwrap();
//         let mayor_events: HashMap<String, Vec<Event>> = read_json_from_file("skyblock_mayor_events.json").unwrap();
//         let mayor_wanted_perks = ["Fishing Festival", "Mining Fiesta", "Mythological Ritual", "Chivalrous Carnival"];
//         for perk in election.perks {
//             if mayor_wanted_perks.contains(&perk.as_str()) {
//                 events.append(&mut mayor_events[perk]);
//             }
//         }
//         for event in &events {
//             if (event.start_time.signed_duration_since(next_valid_skyblock_day).num_seconds() % event.interval as i64) == 0 {
//                 let mut new_event = event.clone();
//                 new_event.description = format!("{:?}", datetime_to_skyblock(next_valid_skyblock_day));
//                 new_event.start_time = next_valid_skyblock_day;
//                 new_event.end_time = next_valid_skyblock_day + Duration::seconds(new_event.duration as i64);
//
//                 calendar.push(new_event);
//                 println!("{}, {}, {:?}",event.name, next_valid_skyblock_day, datetime_to_skyblock(next_valid_skyblock_day))
//             }
//
//         }
//         next_valid_skyblock_day += Duration::minutes(20);
//     }
//     calendar
// }





