use std::net::{TcpListener};
use std::thread;
use std::io::{self};
use log::{error, info};
use crate::request_handler::handle_request;

pub fn init_api() -> io::Result<()> {
    info!("Server started on port {}", 7878);
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New connection: {}", stream.peer_addr()?);
                thread::spawn(|| handle_request(stream));
            }
            Err(e) => {
                error!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}

