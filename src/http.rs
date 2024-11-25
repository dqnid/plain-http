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

#[derive(Debug)]
struct HttpRequestQuery<'a> {
    path: &'a str,
    params: QueryParams,
}

#[derive(Debug)]
struct HttpRequestLine<'a> {
    method: &'a str,
    version: &'a str,
    query: HttpRequestQuery<'a>,
}

#[derive(Debug)]
struct HttpRequest<'a> {
    request: HttpRequestLine<'a>,
    headers: Headers,
    body: Option<String>,
}

pub fn process_petition(stream: &mut TcpStream) -> std::io::Result<ProcessedResponse> {
    let mut buffer = [0; 1024]; // TODO: manage this size

    let _amount = stream.read(&mut buffer)?;

    let petition = String::from_utf8_lossy(&buffer[..]);

    let petition = parse_request(&petition);

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

fn parse_request(request_raw: &str) -> Result<HttpRequest, i16> {
    // TODO: study if better to use match
    if let Some((heading, rest)) = request_raw.split_once("\n") {
        // Process heading
        // split heading with split_whitespace
        // for (i, line) in request_raw.enumerate() {
        // }
        let request = parse_request_block(heading);
        println!("This is a raw request: {:?}", request);
        if let Some((headers, body)) = rest.split_once("\n\n") {
            // Process headers and body
            // split headers over ":"
        }
    }

    Err(400)
}

fn parse_request_block(request_block: &str) -> Result<HttpRequestLine, i16> {
    let [method, query, version]: [&str; 3] = request_block
        .split_whitespace()
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap(); // FIXME: check this out

    if let Ok(query) = parse_query(query) {
        return Ok(HttpRequestLine {
            method,
            version,
            query,
        });
    }
    Err(400)
}

fn parse_query(query: &str) -> Result<HttpRequestQuery, i16> {
    if let Some((path, params)) = query.split_once("?") {
        if let Ok(params) = parse_query_params(params) {
            return Ok(HttpRequestQuery { path, params });
        }
    };

    match query.split_once("?") {
        Some((path, params)) => {
            if let Ok(params) = parse_query_params(params) {
                return Ok(HttpRequestQuery { path, params });
            }
        }
        None => {
            return Ok(HttpRequestQuery {
                path: query,
                params: HashMap::new(),
            })
        }
    }
    Err(400)
}

fn parse_query_params(query: &str) -> Result<QueryParams, i16> {
    Ok(HashMap::new())
    // Err(400)
}
