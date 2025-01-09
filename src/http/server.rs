use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;

use super::*;

struct HTTPServer<'a> {
    config: HttpAppConfig,
    routes: Vec<HttpAppRoute<'a>>,
}

impl HTTPServer<'_> {
    fn get_route(&self, path: &str) -> Option<&HttpAppRoute> {
        self.routes.first() // TODO: search the real one
    }

    fn process_petition(&self, stream: &mut TcpStream) -> ProcessedResponse {
        let mut buffer = [0; 1024]; // TODO: manage this size
        let _amount = stream.read(&mut buffer);
        let petition = String::from_utf8_lossy(&buffer[..]);
        let petition = parse_request(&petition);

        match petition {
            Ok(petition_parsed) => {
                let response_status = "200 OK";

                let response_content = fs::read_to_string("./routes/index.html").unwrap();
                let route = self.get_route(petition_parsed.request.query.path);

                if let Some(route) = route {
                    // TODO: call function and generate response
                } else {
                    // TODO: return not found
                }

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
}
