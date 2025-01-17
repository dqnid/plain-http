use std::collections::HashMap;

/*
 * App types
 * */
pub struct HttpAppConfig {
    pub port: u16,
    pub max_request_size_bytes: usize, // 0 to not max
}

pub type HttpAppRouteFunction = Box<fn(HttpRequest) -> HttpAppRouteResponse>;

pub struct HttpAppRoute {
    pub route: String,
    pub action: HttpAppRouteFunction,
}

pub struct HttpApp<'a> {
    pub config: HttpAppConfig,
    pub routes: Vec<HttpAppRoute>,
    pub default_headers: Headers<'a>,
}

pub struct HttpAppRouteResponse<'a> {
    pub body: String,
    pub content_type: &'a str,
    pub status: Status,
    pub headers: Headers<'a>,
}

/*
 * Parser types
 * */
// TODO:
//  - we could complement status with a struct that stores the status and the error message.
//  - an alternative is to have a status-error_mesage mapper in order to send an error explanation to the client
pub type Status = u16;
pub type Body<'a> = Option<&'a str>;

#[derive(Debug)]
pub struct ProcessedResponse {
    pub data: String,
    pub status: Status, // NOTE: this is dupped, but might be useful so I'll keep it
}

pub type QueryParams<'a> = HashMap<&'a str, &'a str>;
pub type Headers<'a> = HashMap<&'a str, &'a str>;

#[derive(Debug)]
pub struct HttpRequestQuery<'a> {
    pub path: &'a str,
    pub params: QueryParams<'a>,
}

#[derive(Debug)]
pub struct HttpRequestLine<'a> {
    pub method: &'a str,
    pub version: &'a str,
    pub query: HttpRequestQuery<'a>,
}

#[derive(Debug)]
pub struct HttpRequest<'a> {
    pub request: HttpRequestLine<'a>,
    pub headers: Headers<'a>,
    pub body: Body<'a>,
}
