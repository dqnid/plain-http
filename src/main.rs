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
        let _amount = _stream.write(response.data.as_bytes()).unwrap();
        _stream.flush().unwrap();
    }
}
