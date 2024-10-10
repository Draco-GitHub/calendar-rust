use std::fs::File;
use std::{io, time};
use std::error::Error;
use std::io::{ErrorKind, Read};
use chrono::{DateTime, Duration, FixedOffset, Local, Timelike, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::calendar::Event;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Election {
    mayor: String,
    minister: String,
    perks: Vec<String>,
    year: i32,
}

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


fn get_next_valid_time(start: DateTime<FixedOffset>, end: DateTime<FixedOffset>) -> Result<DateTime<FixedOffset>, Box<dyn Error>> {
    let minute = start.minute();
    let valid_minutes = [15, 35, 55];

    for &valid_minute in &valid_minutes {
        if minute < valid_minute {
            let next_time = start.with_minute(valid_minute).unwrap().with_second(0).unwrap();
            if next_time >= end {
                return Err("Next valid time is beyond the end time.".into());
            }
            return Ok(next_time);
        }
    }
    let next_time = start.checked_add_signed(Duration::hours(1)).unwrap().with_minute(15).unwrap().with_second(0).unwrap();
    if next_time >= end {
        return Err("Next valid time is beyond the end time.".into());
    }
    Ok(next_time)
}

pub fn get_skyblock_events(start: DateTime<FixedOffset>, end:DateTime<FixedOffset>) -> Vec<Event> {
    let mut calendar = Vec::new();
    let events = crate::calendar::get_events("skyblock_events.json");
    let mut next_valid_skyblock_day = get_next_valid_time(start, end).unwrap();

    while next_valid_skyblock_day < end {
        for event in &events {
            if (event.start_time.signed_duration_since(next_valid_skyblock_day).num_seconds() % event.interval as i64) == 0 {
                let mut new_event = event.clone();
                new_event.description = format!("{:?}", datetime_to_skyblock(next_valid_skyblock_day));
                new_event.start_time = next_valid_skyblock_day;
                new_event.end_time = next_valid_skyblock_day + Duration::seconds(new_event.duration as i64);

                calendar.push(new_event);
                println!("{}, {}, {:?}",event.name, next_valid_skyblock_day, datetime_to_skyblock(next_valid_skyblock_day))
            }

        }
        next_valid_skyblock_day += Duration::minutes(20);
    }
    calendar
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


