use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};

use super::*;

impl Default for HttpAppConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            max_buffer_size_bytes: 5120,
        }
    }
}

impl Default for HttpApp<'_> {
    fn default() -> Self {
        Self {
            config: Default::default(),
            routes: vec![],
            default_headers: HashMap::new(),
        }
    }
}

impl HttpApp<'_> {
    fn get_route(&self, _path: &str) -> Option<&HttpAppRoute> {
        self.routes.first() // TODO: search the real one
    }

    pub fn add_route(&mut self, route: HttpAppRoute) {
        self.routes.push(route);
    }

    pub fn process_petition(&self, stream: &mut TcpStream) -> ProcessedResponse {
        let mut buffer = [0; 1024]; // TODO: manage this size
        let _amount = stream.read(&mut buffer);
        let petition = String::from_utf8_lossy(&buffer[..]);
        let petition = parse_request(&petition);

        match petition {
            Ok(petition_parsed) => {
                // let mut response_content = fs::read_to_string("./routes/index.html").unwrap();
                if let Some(route) = self.get_route(petition_parsed.request.query.path) {
                    let matched_route = (route.action)(petition_parsed);
                    return format_response(matched_route);
                } else {
                    // TODO: return not found
                    return ProcessedResponse {
                        data: "".to_string(),
                        status: 400,
                    };
                }
            }
            Err(error) => ProcessedResponse {
                data: format!("HTTP/1.1 {}\r\nContent-Length: 0\r\n\r\n", error),
                status: error,
            },
        }
    }

    pub fn start(&self) {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.config.port));

        let listener = TcpListener::bind(addr).unwrap();

        println!("Server up and running!");

        for stream in listener.incoming() {
            let mut _stream = stream.unwrap();
            println!("Connection established!");
            let response = self.process_petition(&mut _stream);

            // TODO: manage error case
            let _amount = _stream.write(response.data.as_bytes()).unwrap();
            _stream.flush().unwrap();
        }
    }
}
