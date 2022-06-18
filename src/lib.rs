use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;

pub fn start_server(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        handle_connection(&mut stream?)?;
    }

    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer)?;

    let contents = fs::read_to_string("pages/hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    println!("{}", response);

    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}