use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Debug, Eq, PartialEq)]
enum Request {
    Publish(String),
    Retrieve
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut storage = VecDeque::new();

    for connection_attempt in listener.incoming() {
        match connection_attempt {
            Ok(stream) => {
                handle_client(stream, &mut storage)
            },
            Err(e) => {
                eprintln!("Error connecting: {}", e)
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, storage: &mut VecDeque<String>) {
    let line = read_line(&stream);
    let request = parse_request(line);

    println!("Client connected!");
    match request {
        Request::Publish(msg) => {
            storage.push_back(msg)
        }
        Request::Retrieve => {
            let maybe_msg = storage.pop_front();
            match maybe_msg {
                Some(msg) => {
                    stream.write_all(msg.as_bytes()).unwrap();
                }
                None => {
                    stream.write_all(b"no message available").unwrap();
                }
            }
        }
    }
}

fn read_line(stream: &TcpStream) -> String {
    let mut buffered_reader = BufReader::new(stream);

    let mut buf = String::new();
    buffered_reader.read_line(&mut buf).unwrap();
    buf
}

fn parse_request(line: String) -> Request {
    let trimmed = line.trim_end();

    if trimmed == "" {
        Request::Retrieve
    } else {
        Request::Publish(String::from(trimmed))
    }
}
