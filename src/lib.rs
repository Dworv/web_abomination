use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub fn start_server(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        handle_connection(&mut stream?);
    }

    Ok(())
}

fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer[..]));
}