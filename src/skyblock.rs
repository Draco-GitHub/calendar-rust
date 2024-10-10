use std::fs::File;
use std::{io};
use std::io::{Read};
use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Election {
    mayor: String,
    minister: String,
    perks: Vec<String>,
    year: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct SkyblockDay {
    day: i32,
    month: i32,
    year: i32,
    events: Vec<SkyblockEvent>
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct SkyblockEvent {
    title: String,
    duration: i32,
    interval: i32,
}
impl SkyblockDay {
    fn new( day: i32, month: i32, year: i32) -> Self {
        let events = Self::get_events(day, month, year);
        SkyblockDay { day, month, year, events}
    }

    fn convert_to_date(&self) -> DateTime<Utc> {
        let year_start: DateTime<Utc> = "2024-09-30 05:55:00".parse().expect("Failed to parse datetime");
        let total_days = ((self.year - 376) * 12 * 31) + ((self.month - 1) * 31) + (self.day - 1);
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

    fn get_events(day:i32, month:i32, year:i32) -> Vec<SkyblockEvent> {
        let events = Vec::new();
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


fn get_election(year:i32) -> Result<Election, io::Error>{
    let mut file = File::open("elections.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read contents");
    let json:Vec<Election>= serde_json::from_str(&contents)?;
    for election in json {
        if election.year == year {
            return Ok(election);
        }
    }
    Err("ElectionError").expect("Election not found")
}


