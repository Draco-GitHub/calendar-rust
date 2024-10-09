use std::fs::File;
use std::{io, time};
use std::io::{ErrorKind, Read};
use chrono::{DateTime, Duration, FixedOffset, Local, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
//
//
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct SkyblockEvent {
//     day: i8,
//     month: i8,
//     year: i16,
//     day_of_year: i16,
//     name: String,
//     start: DateTime<FixedOffset>,
//     end: DateTime<FixedOffset>,
// }
// #[derive(Debug, Serialize, Deserialize, Clone)]
// struct Election {
//     mayor: String,
//     minister: String,
//     perks: Vec<String>,
//     year: i16,
// }
//
//
//
//
pub fn skyblock_to_datetime(day:i32, month:i32, year:i32) -> DateTime<FixedOffset>{
    let year_start: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2024-09-30T06:55:00+01:00").expect("Failed to parse time");

    let total_days = ((year - 376) * 12 * 31) + ((month - 1) * 31) as i32 + (day - 1) as i32;
    let total_real_minutes = total_days * 20;
    let real_duration = Duration::minutes(total_real_minutes.into());
    year_start + real_duration
}


pub fn datetime_to_skyblock(real_date: DateTime<FixedOffset>) -> (i32, i32, i32) {
    let year_start = DateTime::parse_from_rfc3339("2024-09-30T06:55:00+01:00")
        .expect("Failed to parse time");

    let time_delta_minutes = real_date.signed_duration_since(year_start).num_minutes();

    let delta_days = time_delta_minutes as f32 / 20.0;
    let delta_years = (delta_days / 372.0).floor();
    let year = 376 + delta_years as i32;

    let remaining_days = delta_days - (delta_years * 372.0);
    let month = (remaining_days / 31.0).floor() as i32 + 1;
    let day = (remaining_days % 31.0).floor() as i32 + 1;
    (day, month, year)
}

// // fn get_current_date(day:CalendarEvent) -> CalendarEvent {
// //     let now = Utc::now().with_timezone(&FixedOffset::east_opt(3600).unwrap());
// //     let mut days_from_now = (now - day.datetime).num_minutes() as f64/20.0;
// //     let days_from_now = if days_from_now < 0.0 {days_from_now.floor() as i32} else {days_from_now.ceil() as i32};
// //     let new_days = (day.day + days_from_now)%31;
// //     let new_month = day.month + (days_from_now as f64/31.0).floor() as i32;
// //     let new_year = day.month + (days_from_now as f64/372.0).floor() as i32;
// //     fill_events_vec(new_days, new_month, new_year, now)
// // }
//
// fn get_election(year:i16) -> Result<Election, io::Error>{
//     let mut file = File::open("elections.json")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).expect("Unable to read contents");
//     let json:Vec<Election>= serde_json::from_str(&contents)?;
//     for election in json {
//         if election.year == year {
//             return Ok(election);
//         }
//     }
//     Err("ElectionError").expect("Election not found")
// }


