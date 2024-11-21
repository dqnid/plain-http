use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct ProcessedResponse {
    pub data: String,
    status: u8,
}

pub fn process_petition(stream: &mut TcpStream) -> std::io::Result<ProcessedResponse> {
    let mut buffer = [0; 1024];

    let _amount = stream.read(&mut buffer)?;

    let petition = String::from_utf8_lossy(&buffer[..]);

    let petition = petition.split("\n");

    for line in petition {
        println!("PART: {}", line)
    }

    let response_status = "200 OK";

    let response_content = fs::read_to_string("./index.html").unwrap();

    let response: ProcessedResponse = ProcessedResponse {
        data: format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
            response_status,
            response_content.len(),
            response_content
        ),
        status: 200,
    };

    Ok(response)
}
