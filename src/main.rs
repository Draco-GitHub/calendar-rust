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
    println!("test: {:?}", SkyblockDay::new(14,3,379).as_datetime());
    println!("election start: {:?}", SkyblockDay::new(30,6,379).as_datetime());
    println!("election end: {:?}", SkyblockDay::new(30,6,380).as_datetime());




    let mut calendar_database = DataBase::new();
    let mut new_user = User::new("Global".to_string());
    let mut new_calendar:Calendar = skyblock::generate_calendar(Utc::now(), Utc::now()+Duration::minutes(7460));
    let calendar_id = new_calendar.get_id();
    new_user.add_calendar(new_calendar);
    let mut calendar = new_user.get_calendar(calendar_id).unwrap();
    //
    calendar_database.add_user(new_user.clone());
    //
    // let mut sbday = SkyblockDay::new(1, 1, 378);
    // let ev = sbday.find_events();
    // println!("{:?}", ev);
    // println!("{:?}", calendar_database.list_users());
    // println!("{:?}", calendar_database.get_user(new_user.clone().get_id()));
    
    init_logger();
    init_api()?;

    Ok(())
}
