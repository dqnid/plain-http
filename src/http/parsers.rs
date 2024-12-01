use std::collections::HashMap;

use super::*;

pub fn parse_request(request_raw: &str) -> Result<HttpRequest, Status> {
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
