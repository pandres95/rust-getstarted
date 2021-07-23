mod rusty_http;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use rusty_http::*;
use serde_json::json;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7373").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut request_buffer = [0; 1024];
    stream.read(&mut request_buffer).unwrap();

    let responded: bool;
    responded = route(&stream, request_buffer, "GET", "/", |_request| {
        json!({
            "body": {
                "hello": "world"
            }
        })
    });

    if !responded {
        route_any(&stream, request_buffer, |_request| {
            json!({
                "code": 404,
                "headers": {
                    "content-type": "text/plain"
                },
                "body": "Not Found"
            })
        });
    }
}
