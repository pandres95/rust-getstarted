use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

use regex::Regex;
use serde_json::Value;

pub fn route<F: Fn([u8; 1024]) -> Value>(
    stream: &TcpStream,
    request: [u8; 1024],
    method: &str,
    path: &str,
    callback: F,
) -> bool {
    let regex = Regex::new(r"^(\w+) (.*) HTTP/1.1\r\n").unwrap();
    let input = str::from_utf8(&request[..]).unwrap();
    if regex.is_match(input) {
        let cap = regex.captures(input).unwrap();
        if &cap[1] == method.to_uppercase() && &cap[2] == path {
            let response = callback(request);
            write_response(&stream, response);
            return true;
        }
    }
    return false;
}

pub fn route_any<F: Fn([u8; 1024]) -> Value>(stream: &TcpStream, request: [u8; 1024], callback: F) {
    let response = callback(request);
    write_response(stream, response);
}

fn write_response(mut stream: &TcpStream, response: Value) {
    let code = response["code"].as_u64().unwrap_or(200);
    let content_type = response["headers"]["content-type"]
        .as_str()
        .unwrap_or("application/json");
    let body = match content_type {
        "text/plain" => String::from(response["body"].as_str().unwrap_or("")),
        _ => response["body"].to_string()
    };

    
    let response = format!(
        "HTTP/1.1 {}
Content-Length: {}
Content-Type: {}\r\n
{}",
        set_status_code(code),
        body.len(),
        content_type,
        body
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}

fn set_status_code(status: u64) -> String {
    let mut status_map: HashMap<u64, &str> = HashMap::new();
    status_map.insert(200, "OK");
    status_map.insert(404, "Not Found");

    return format!("{} {}", status, status_map.get(&status).unwrap());
}
