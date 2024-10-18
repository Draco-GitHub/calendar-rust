mod api;
mod request_handler;
mod calendar;
mod skyblock;
mod helpers;

use crate::calendar::{Calendar, DataBase, Event, User};
use api::init_api;
use chrono::prelude::*;
use env_logger::Builder;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use std::io;
use chrono::Duration;
use crate::skyblock::{generate_calendar, SkyblockDay};

fn init_logger() {
    let log_file_path = "server.log";
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .unwrap();

    let file = Mutex::new(file);

    Builder::new()
        .format(move |buf, record| {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            writeln!(buf, "{} [{}] - {}", timestamp, record.level(), record.args())?;
            let mut file = file.lock().unwrap();
            writeln!(file, "{} [{}] - {}", timestamp, record.level(), record.args())
        })
        .filter(None, log::LevelFilter::Info)
        .init();
}

fn main() -> io::Result<()> {
    // println!("test: {:?}", SkyblockDay::new(14,3,379).as_datetime());
    // println!("election start: {:?}", SkyblockDay::new(30,6,379).as_datetime());
    // println!("election end: {:?}", SkyblockDay::new(30,6,380).as_datetime());
    //
    // let x = DateTime::parse_from_str("2024-10-17 13:55:00+01:00", "%Y-%m-%d %H:%M:%S%z").expect("no x");
    // let y = DateTime::parse_from_str("2024-10-22 04:15:00+01:00", "%Y-%m-%d %H:%M:%S%z").expect("no y");
    // let z = y-x;
    //
    // let (days, hours, minutes) = seconds_to_dhm(z.num_seconds());


    let calendar_database = DataBase::new();
    println!("{:?}", calendar_database.list_users());

    init_logger();
    init_api()?;

    Ok(())
}

fn seconds_to_dhm(seconds: i64) -> (i64, i64, i64) {
    let days = seconds / 86400; // 86400 seconds in a day
    let hours = (seconds % 86400) / 3600; // 3600 seconds in an hour
    let minutes = (seconds % 3600) / 60; // 60 seconds in a minute
    (days, hours, minutes)
}
