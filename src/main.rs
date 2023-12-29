use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n";

fn main() {
    println!("Listening on port 4221");

    let listener = TcpListener::bind("127.0.0.1:4221").expect("Failed to bind port");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("new client!");
    let buffer = BufReader::new(&stream);
    let request: Vec<_> = buffer
        .lines()
        .map(|result| result.unwrap_or_default())
        .take_while(|line| !line.is_empty())
        .collect();
    let path = request[0].split_whitespace().nth(1).unwrap_or_default();
    let user_agent = request[2].split_whitespace().nth(1).unwrap_or_default();
    match path {
        path if path.starts_with("/echo/") => handle_manual_route(stream, &path[6..])?,
        path if path.starts_with("/user-agent") => handle_manual_route(stream, user_agent)?,
        "/" => stream.write_all(format!("{OK_RESPONSE}\r\n").as_bytes())?,
        _ => stream.write_all(format!("{NOT_FOUND}\r\n").as_bytes())?,
    };
    Ok(())
}

fn handle_manual_route(mut stream: TcpStream, thing_to_echo: &str) -> Result<(), Box<dyn Error>> {
    let length = thing_to_echo.len();
    let response = format!("{OK_RESPONSE}Content-type: text/plain\r\nContent-Length: {length}\r\n\r\n{thing_to_echo}\r\n");
    stream.write_all(response.as_bytes())?;
    Ok(())
}
