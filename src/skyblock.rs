use std::fs::File;
use std::{io, time};
use std::io::{ErrorKind, Read};
use chrono::{DateTime, Duration, FixedOffset, Local, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkyblockEvent {
    day: i8,
    month: i8,
    year: i16,
    day_of_year: i16,
    name: String,
    start: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Election {
    mayor: String,
    minister: String,
    perks: Vec<String>,
    year: i16,
}




pub fn skyblock_to_datetime(day:i8, month:i8, year:i16) -> DateTime<FixedOffset>{
    let year_start: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2024-09-30T06:55:00+01:00").expect("Failed to parse time");

    let total_days = ((year - 376) * 12 * 31) + ((month - 1) * 31) as i16 + (day - 1) as i16;
    let total_real_minutes = total_days * 20;
    let real_duration = Duration::minutes(total_real_minutes.into());
    year_start + real_duration
}


pub fn datetime_to_skyblock(real_date: DateTime<FixedOffset>) -> (i8, i8, i16) {
    let year_start = DateTime::parse_from_rfc3339("2024-09-30T06:55:00+01:00")
        .expect("Failed to parse time");

    let time_delta_minutes = real_date.signed_duration_since(year_start).num_minutes();

    let delta_days = time_delta_minutes as f64 / 20.0;
    let delta_years = (delta_days / 372.0).floor();
    let year = 376 + delta_years as i16;

    let remaining_days = delta_days - (delta_years * 372.0);
    let month = (remaining_days / 31.0).floor() as i8 + 1;
    let day = (remaining_days % 31.0).floor() as i8 + 1;
    (day, month, year)
}

// fn get_current_date(day:CalendarEvent) -> CalendarEvent {
//     let now = Utc::now().with_timezone(&FixedOffset::east_opt(3600).unwrap());
//     let mut days_from_now = (now - day.datetime).num_minutes() as f64/20.0;
//     let days_from_now = if days_from_now < 0.0 {days_from_now.floor() as i32} else {days_from_now.ceil() as i32};
//     let new_days = (day.day + days_from_now)%31;
//     let new_month = day.month + (days_from_now as f64/31.0).floor() as i32;
//     let new_year = day.month + (days_from_now as f64/372.0).floor() as i32;
//     fill_events_vec(new_days, new_month, new_year, now)
// }

fn get_events_from_day(day:i8, month:i8, year:i16, election: Election) -> Vec<SkyblockEvent> {
    let datetime = skyblock_to_datetime(day, month, year);
    let day_of_year:i16 = (day + month * 31) as i16;
    let mut events:Vec<SkyblockEvent>  = Vec::new();

    if day_of_year % 3 == 0 {
        events.push(SkyblockEvent { day, month, year, day_of_year, name: "Jacob's Farming Contest".to_string(), start:datetime, end:datetime + Duration::minutes(20)})
    }
    if day_of_year % 3 == 1 {
        events.push(SkyblockEvent { day, month, year, day_of_year , name: "Dark Auction".to_string(), start:datetime, end:datetime + Duration::minutes(20)})
    }

    if day == 1 {
        if (election.perks.contains(&"Fishing Festival".to_string())) {
            if ((1..=3).contains(&month) && year == election.year) || (5..=12).contains(&month) {
                events.push(SkyblockEvent { day, month, year, day_of_year, name:"Fishing Festival".to_string(), start:datetime, end:datetime+Duration::hours(1)})
            }
        }
        if month == 1 {
            events.push(SkyblockEvent { day, month, year, day_of_year, name: "Hoppity Start".to_string(), start:datetime, end:datetime + Duration::hours(31)})
        }
        if month == 4 || month == 10 {
            events.push(SkyblockEvent { day, month, year, day_of_year, name: "Traveling Zoo".to_string(), start:datetime, end:datetime + Duration::hours(1)})
        }
        if month == 12 {
            events.push(SkyblockEvent { day, month, year, day_of_year, name: "Jerry Workshop Opens".to_string(), start:datetime, end:datetime + Duration::minutes(620)})
        }
    }

    if month == 12 {
        if day == 25 {
            events.push(SkyblockEvent { day, month, year, day_of_year, name:"Season of Jerry".to_string(), start:datetime, end:datetime + Duration::minutes(620)})
        }
        if day == 29 {
            events.push(SkyblockEvent { day, month, year, day_of_year, name:"New Year Celebration".to_string(), start:datetime, end:datetime + Duration::minutes(620)})
        }
    }

    if month == 8 && day == 29 {
            events.push(SkyblockEvent { day, month, year, day_of_year, name: "Spooky Festival".to_string(), start:datetime, end:datetime + Duration::hours(1)})
    }

    events
}

pub async fn get_calendar() -> io::Result<Vec<SkyblockEvent>> {

    let mut file = File::open("calendar2.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read contents");
    let json: Vec<SkyblockEvent> = serde_json::from_str(&contents)?;

    let last_day = &json[json.len()-1];
    let now = Utc::now().with_timezone(&FixedOffset::east_opt(3600).unwrap());
    let mut days_from_now = (now - last_day.start).num_minutes() as f64/20.0;

    let days_from_now = if days_from_now < 0.0 {days_from_now.floor()} else {days_from_now.ceil()};

    println!("{days_from_now}");

    let client = Client::new();

    let response = client
        .get("https://api.hypixel.net/v2/resources/skyblock/election")
        .send()
        .await
        .map_err(|e| io::Error::new(ErrorKind::Other, e))?
        .json::<Value>()
        .await
        .map_err(|e| io::Error::new(ErrorKind::Other, e))?;

    let mut calendar: Vec<SkyblockEvent> = Vec::new();


    Ok(calendar)
}


pub async fn notify_before_event() -> io::Result<()> {
    loop {
        let calendar = get_calendar().await?;

        let local_time: DateTime<Local> = Local::now();
        let now: DateTime<FixedOffset> = local_time.with_timezone(local_time.offset());
        let (year, month, day) = datetime_to_skyblock(now);

        let current_day_of_year = (month - 1) * 31 + day;

        if current_day_of_year < calendar.len() as i32 {
            let next_day = &calendar[(current_day_of_year % calendar.len() as i32) as usize];
            let next_event_datetime = skyblock_to_datetime(year, month, day+1);
            let notify_time = next_event_datetime - Duration::minutes(2);
            let seconds_till_event = notify_time.signed_duration_since(now).num_seconds();
            println!("{seconds_till_event}");
            if seconds_till_event < 0 {
                println!("retrying");
                continue
            }
            println!("Notifying");

            // Notification::new()
            //     .summary("Skyblock Event Reminder")
            //     .body(&format!("The event(s) '{}' is starting in 2 minutes!", next_day.events.join(", ")))
            //     .show()
            //     .unwrap();

        } else {
            println!("There are no more days in the calendar.");
        }
        tokio::time::sleep(time::Duration::from_secs(1200)).await;
    }
}




