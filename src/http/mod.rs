use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;

mod generators;
mod parsers;
mod types;

use parsers::*;
use types::*;

pub fn process_petition(stream: &mut TcpStream) -> ProcessedResponse {
    let mut buffer = [0; 1024]; // TODO: manage this size
    let _amount = stream.read(&mut buffer);
    let petition = String::from_utf8_lossy(&buffer[..]);
    let petition = parse_request(&petition);

    match petition {
        Ok(petition_parsed) => {
            let response_status = "200 OK";

            let response_content = fs::read_to_string("./routes/index.html").unwrap();

            let response: ProcessedResponse = ProcessedResponse {
                data: format!(
                    "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
                    response_status,
                    response_content.len(),
                    response_content
                ),
                status: 200,
            };

            response
        }
        Err(error) => {
            let response: ProcessedResponse = ProcessedResponse {
                data: format!("HTTP/1.1 {}\r\nContent-Length: 0\r\n\r\n", error),
                status: error,
            };

            response
        }
    }
}
