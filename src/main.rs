use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};

fn handle_petition(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];

    let amount = stream.read(&mut buffer)?;

    println!(
        "Request: {} of {amount} bytes",
        String::from_utf8_lossy(&buffer[..])
    );
    Ok(())
}

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 80));

    let listener = TcpListener::bind(addr).unwrap();

    println!("Server up and running!");

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established!");
        let _result = handle_petition(_stream);
    }
}
