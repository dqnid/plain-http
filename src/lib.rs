use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener};

mod http;

fn principal() {
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

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
