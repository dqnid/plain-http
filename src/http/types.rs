use std::collections::HashMap;

/*
 * App types
 * */
pub struct HttpAppConfig {
    port: u8,
}

pub type HttpAppRouteFunction = Box<dyn Fn(HttpRequest) -> String>;

pub struct HttpAppRoute<'a> {
    route: &'a str,
    action: HttpAppRouteFunction,
}

pub struct HttpApp<'a> {
    config: HttpAppConfig,
    routes: Vec<HttpAppRoute<'a>>,
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
