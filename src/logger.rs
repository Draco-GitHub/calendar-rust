use chrono::Local;
use env_logger::Builder;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

pub fn init_logger() {
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
            writeln!(
                buf,
                "{} [{}] - {}",
                timestamp,
                record.level(),
                record.args()
            )?;
            let mut file = file.lock().unwrap();
            writeln!(
                file,
                "{} [{}] - {}",
                timestamp,
                record.level(),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Info)
        .init();
}

