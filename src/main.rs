use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_path_echo(path: &str, _stream: &TcpStream) -> String {
    if let Some((_prefix, content)) = path.split_once("/echo/") {
        let content_length = content.as_bytes().len();
        let content_type = "text/plain";
        return format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            content_type, content_length, content
        );
    }
    return format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
}

fn handle_path_user_agent(user_agent: &str, _stream: &TcpStream) -> String {
    return format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        user_agent.len(),
        user_agent
    );
}

fn handle_stream(mut _stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _bytes_read = _stream.read(&mut buffer).unwrap();
    let data = std::str::from_utf8(&buffer[.._bytes_read]).expect("Invalid UTF-8");
    let request_line: Vec<&str> = data.lines().take(3).collect();

    let (path, _host, agent) = (request_line[0], request_line[1], request_line[2]);
    let path_line: Vec<&str> = path.split_whitespace().collect();

    let response = if path_line[1].starts_with("/echo/") {
        handle_path_echo(path_line[1], &_stream)
    } else if path_line[1].starts_with("/user-agent") {
        handle_path_user_agent(agent.split_whitespace().last().unwrap(), &_stream)
    } else if path_line[1] == "/" {
        format!("HTTP/1.1 200 OK\r\n\nContent-Type: text/html\r\nContent-Length: 0\r\n\r\n")
    } else {
        format!("HTTP/1.1 404 NOT FOUND\r\n\r\n")
    };

    _stream.write(response.as_bytes()).unwrap();
}

fn main() {
    //  Binding the listener to the (host and port)
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    // Listening for incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                thread::spawn(|| handle_stream(_stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
