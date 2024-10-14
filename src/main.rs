mod api;
mod request_handler;
mod calendar;
mod skyblock;
mod helpers;

use crate::calendar::{CalendarDataBase, User};
use api::init_api;
use chrono::prelude::*;
use env_logger::Builder;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use std::io;

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
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(async {
    //     if let Err(e) = notify_before_event().await {
    //         eprintln!("Error {:?}", e)
    //     }
    // });

    // let start = DateTime::parse_from_rfc3339("2024-10-10T13:00:00+01:00").expect("Failed to parse time");
    // let end  = DateTime::parse_from_rfc3339("2024-10-17T12:55:00+01:00").expect("Failed to parse time");
    // let calendar = get_calendar(start, end);

    let mut calendar_database = CalendarDataBase::new();
    let new_user = User::new("Draco".to_string());
    calendar_database.add_user(new_user);
    println!("{:?}", calendar_database.list_users());
    init_logger();
    init_api()?;

    Ok(())
}
