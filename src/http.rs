use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct ProcessedResponse {
    pub data: String,
    status: u8,
}

type QueryParams = HashMap<String, String>;
type Headers = HashMap<String, String>;

struct HttpRequestQuery {
    path: String,
    params: QueryParams,
}

struct HttpRequestLine {
    method: String,
    version: f32,
    query: HttpRequestQuery,
}

struct HttpRequest {
    request: HttpRequestLine,
    headers: Headers,
    body: String,
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

fn parse_request(request_raw: String) -> Result<HttpRequest, i16> {
    // TODO: study if better to use match
    if let Some((heading, rest)) = request_raw.split_once("\n") {
        // Process heading
        // split heading with split_whitespace
        // for (i, line) in request_raw.enumerate() {
        // }
        if let Some((headers, body)) = rest.split_once("\n\n") {
            // Process headers and body
            // split headers over ":"
        }
    }

    Err(400)
}

fn parse_request_block(request_block: &str) -> Result<HttpRequestLine, i16> {
    let request_components: Vec<&str> = request_block.split(" ").collect();

    if let Ok(array) = request_components.try_into() {
        if let Ok(query) = parse_query(query) {
            return Ok(HttpRequestLine {
                method,
                version,
                query,
            });
        }
    }
    Err(400)
}

fn parse_query(query: &str) -> Result<HttpRequestQuery, i16> {
    if let Some((path, params)) = query.split_once("?") {
        return Ok(HttpRequestQuery {
            path: path.to_string(),
            params: parse_query_params(params),
        });
    };
    Err(400)
}

fn parse_query_params(query: &str) -> QueryParams {}
