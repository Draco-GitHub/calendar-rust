mod api;
mod request_handler;
mod calendar;
mod skyblock;

use std::fs::OpenOptions;
use api::init_api;
use std::io::Write;
use std::{io};
use std::sync::Mutex;
use chrono::prelude::*;
use env_logger::Builder;
use crate::calendar::{notify_before_event};

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
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        if let Err(e) = notify_before_event().await {
            eprintln!("Error {:?}", e)
        }
    });

    init_logger();
    init_api()?;

    Ok(())
}
