use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;

// TODO:
//  - we could complement status with a struct that stores the status and the error message.
//  - an alternative is to have a status-error_mesage mapper in order to send an error explanation to the client
type Status = u16;
type Body<'a> = Option<&'a str>;

#[derive(Debug)]
pub struct ProcessedResponse {
    pub data: String,
    status: Status,
}

type QueryParams<'a> = HashMap<&'a str, &'a str>;
type Headers<'a> = HashMap<&'a str, &'a str>;

#[derive(Debug)]
struct HttpRequestQuery<'a> {
    path: &'a str,
    params: QueryParams<'a>,
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
    headers: Headers<'a>,
    body: Body<'a>,
}

pub fn process_petition(stream: &mut TcpStream) -> ProcessedResponse {
    let mut buffer = [0; 1024]; // TODO: manage this size
    let _amount = stream.read(&mut buffer);
    let petition = String::from_utf8_lossy(&buffer[..]);
    println!("Petition: {:?}", petition);
    let petition = parse_request(&petition);

    match petition {
        Ok(petition_parsed) => {
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

fn parse_request(request_raw: &str) -> Result<HttpRequest, Status> {
    // TODO: study if better to use match
    if let Some((heading, rest)) = request_raw.split_once("\n") {
        if let Ok(request) = parse_request_block(heading) {
            if let Some((headers, body)) = rest.split_once("\n\r\n") {
                if let Ok(headers) = parse_headers(headers) {
                    let body: Body = {
                        if body.len() > 0 {
                            Some(body)
                        } else {
                            None
                        }
                    };
                    return Ok(HttpRequest {
                        request,
                        headers,
                        body,
                    });
                }
            }
        }
    }

    Err(400)
}

fn parse_request_block(request_block: &str) -> Result<HttpRequestLine, Status> {
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

fn parse_query(query: &str) -> Result<HttpRequestQuery, Status> {
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

fn parse_query_params(query: &str) -> Result<QueryParams, Status> {
    let mut param_map: HashMap<&str, &str> = HashMap::new();

    let param_list = query.split("&");

    for param in param_list {
        if let Some((key, param)) = param.split_once("=") {
            param_map.insert(key, param);
        } else {
            return Err(400);
        }
    }

    Ok(param_map)
}

fn parse_headers(headers: &str) -> Result<Headers, Status> {
    let mut header_map: Headers = HashMap::new();

    let header_list = headers.split("\n");

    for header in header_list {
        if let Some((key, value)) = header.split_once(":") {
            header_map.insert(key, value);
        } else {
            return Err(400);
        }
    }

    Ok(header_map)
}
