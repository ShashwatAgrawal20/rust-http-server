use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Listening on port 4221");

    let listener = TcpListener::bind("127.0.0.1:4221").expect("Failed to bind port");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    eprintln!("error: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    println!("new client!");
    let buffer = BufReader::new(&stream);
    let request: Vec<_> = buffer
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let get_first_header: Vec<&str> = request[0].split(" ").collect();
    let path = get_first_header[1];
    match path {
        "/" => stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?,
        _ => stream.write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())?,
    };
    Ok(())
}
