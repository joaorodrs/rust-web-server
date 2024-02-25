use std::{io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    println!("Ready to connect!");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connections(stream)
    }
}

fn handle_connections(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
