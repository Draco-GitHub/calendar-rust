use std::fs::File;
use std::io::BufReader;
use serde::de::{DeserializeOwned, Error};

pub fn read_json_from_file<T: DeserializeOwned>(path: &str) -> Result<T, serde_json::Error> {
    let file = File::open(path).map_err(|e| serde_json::Error::custom(format!("File error: {}", e)))?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}