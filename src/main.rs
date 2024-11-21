use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener};

mod http;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 80));

    let listener = TcpListener::bind(addr).unwrap();

    println!("Server up and running!");

    for stream in listener.incoming() {
        let mut _stream = stream.unwrap();
        println!("Connection established!");
        let response = http::process_petition(&mut _stream);

        // TODO: manage error case
        match response {
            Ok(data) => {
                let _amount = _stream.write(data.data.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("Error: {:?}", e);
                let _amount = _stream
                    .write(
                        "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n"
                            .as_bytes(),
                    )
                    .unwrap();
            }
        }
        _stream.flush().unwrap();
    }
}
