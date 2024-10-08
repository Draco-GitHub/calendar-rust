use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json::Value;
use log::{error, info};

fn handle_response(mut stream: TcpStream, status: &str, response_str: &str, content_type: &str) {
    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: close\r\n\r\n{}",
        status,
        response_str.len(),
        content_type,
        response_str
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        error!("Error Writing response: {}", e);
    }

    if let Err(e) = stream.flush() {
        error!("Error flushing stream: {}", e);
    }
}

fn handle_post(stream: TcpStream, request: &str) {
    let content_length = get_content_length(&request).unwrap_or(0);

    if content_length > 0 {
        let body_start = request.find("\r\n\r\n").map(|pos| pos + 4).unwrap_or(0);
        let body_str = &request[body_start..body_start + content_length];

        if body_str.len() != content_length {
            error!("Error reading body: expected: {} bytes, got {}", content_length, body_str.len());
            handle_response(
                stream,
                "HTTP/1.1 500 Internal Server Error",
                "{\"error\": \"Failed to read body\"}",
                "application/json",
            );
            return;
        }

        match serde_json::from_str::<Value>(body_str) {
            Ok(json_value) => {
                info!("Received JSON: {}", json_value);
                handle_response(
                    stream,
                    "HTTP/1.1 200 OK",
                    "{\"success\": \"JSON received\"}",
                    "application/json",
                );
            }
            Err(_) => {
                error!("Error Invalid Json");
                handle_response(
                    stream,
                    "HTTP/1.1 400 Bad Request",
                    "{\"error\": \"Invalid JSON\"}",
                    "application/json",
                );
            }
        }
    } else {
        error!("Error content-Length is zero or not found.");
        handle_response(
            stream,
            "HTTP/1.1 400 Bad Request",
            "{\"error\": \"No content\"}",
            "application/json",
        );
    }
}

pub fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Err(e) = stream.read(&mut buffer) {
        error!("Error requesting request {}", e);
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..]);

    if request.starts_with("GET /") {
        handle_response(
            stream,
            "HTTP/1.1 200 OK",
            "{\"success\": \"GET request handled\"}",
            "application/json",
        );
    } else if request.starts_with("POST /") {
        handle_post(stream, &request);
    } else {
        handle_response(
            stream,
            "HTTP/1.1 400 Bad Request",
            "{\"error\": \"Unsupported request\"}",
            "application/json",
        );
    }
}

fn get_content_length(request: &str) -> Option<usize> {
    for line in request.lines() {
        if line.starts_with("Content-Length:") {
            return line[15..].trim().parse::<usize>().ok();
        }
    }
    None
}
