use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
    {env, fs, thread},
};

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n";
const CREATED: &str = "HTTP/1.1 201 OK\r\n";

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
    let mut buff_reader = BufReader::new(&stream);
    let buffer: &[u8] = buff_reader.fill_buf()?;
    let result: String = String::from_utf8_lossy(buffer).to_string();
    let args: Vec<String> = env::args().collect();
    let request: Vec<_> = buff_reader
        .lines()
        .map(|result| result.unwrap_or_default())
        .take_while(|line| !line.is_empty())
        .collect();
    let method = request[0].split_whitespace().nth(0).unwrap_or_default();
    let path = request[0].split_whitespace().nth(1).unwrap_or_default();
    let user_agent = request[2].split_whitespace().nth(1).unwrap_or_default();
    match path {
        path if path.starts_with("/echo/") => handle_manual_route(stream, &path[6..])?,
        path if path.starts_with("/user-agent") => handle_manual_route(stream, user_agent)?,
        path if path.starts_with("/files") && method.starts_with("POST") && args.len() > 1 => {
            post_file(stream, &path[7..], args, result)?
        }
        path if path.starts_with("/files") && args.len() > 1 => get_file(stream, &path[7..], args)?,
        "/" => stream.write_all(format!("{OK_RESPONSE}\r\n").as_bytes())?,
        _ => stream.write_all(format!("{NOT_FOUND}\r\n").as_bytes())?,
    };
    Ok(())
}

fn get_file(mut stream: TcpStream, path: &str, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let file_path = args[2].to_string();
    let file_name = path;
    let absolute_path = format!("{file_path}/{file_name}");
    if Path::new(&absolute_path).exists() && Path::new(&absolute_path).is_file() {
        let content = fs::read_to_string(&absolute_path)?;
        let length = content.len();
        let response = format!("{OK_RESPONSE}Content-type: text/plain\r\nContent-Length: {length}\r\n\r\n{content}\r\n");
        stream.write_all(response.as_bytes())?;
    } else {
        stream.write_all(format!("{NOT_FOUND}\r\n").as_bytes())?;
    }
    Ok(())
}

fn post_file(
    mut stream: TcpStream,
    path: &str,
    args: Vec<String>,
    result: String,
) -> Result<(), Box<dyn Error>> {
    let file_path = args[2].to_string();
    let file_name = path;
    let absolute_path = format!("{file_path}/{file_name}");
    let body: String = result
        .splitn(2, "\r\n\r\n")
        .nth(1)
        .map(|s| s.to_string())
        .unwrap_or_default();
    if Path::new(&file_path).is_dir() && body.len() > 0 {
        let mut file = File::create(absolute_path)?;
        file.write_all(body.as_bytes())?;
        stream.write_all(format!("{CREATED}\r\n").as_bytes())?;
    } else {
        stream.write_all(format!("{NOT_FOUND}\r\n").as_bytes())?;
    }
    Ok(())
}

fn handle_manual_route(mut stream: TcpStream, thing_to_echo: &str) -> Result<(), Box<dyn Error>> {
    let length = thing_to_echo.len();
    let response = format!("{OK_RESPONSE}Content-type: text/plain\r\nContent-Length: {length}\r\n\r\n{thing_to_echo}\r\n");
    stream.write_all(response.as_bytes())?;
    Ok(())
}
