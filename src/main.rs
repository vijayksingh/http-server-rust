use std::io::{Read, Write};
use std::net::TcpListener;
fn main() {
    //  Binding the listener to the (host and port)
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let mut buffer = [0; 1024];
    // Listening for incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let _bytes_read = _stream.read(&mut buffer).unwrap();
                let data = std::str::from_utf8(&buffer[.._bytes_read]).expect("Invalid UTF-8");
                let request_line: Vec<&str> = data.lines().take(3).collect();

                let (path, _host, agent) = (request_line[0], request_line[1], request_line[2]);
                let mut response: String = String::from("");
                let path_line: Vec<&str> = path.split_whitespace().collect();
                let user_agent = agent.split_whitespace().last().unwrap();

                let prefix = "/echo/";
                if path_line[1].starts_with(prefix) {
                    if let Some((_prefix, content)) = path_line[1].split_once(prefix) {
                        let content_length = content.as_bytes().len();
                        let content_type = "text/plain";
                        response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                            content_type, content_length, content
                        )
                    }
                } else if path_line[1].starts_with("/user-agent") {
                    response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                        user_agent.len(),
                        user_agent
                    );
                } else if path_line[1] == "/" {
                    response = format!(
                        "HTTP/1.1 200 OK\r\n\nContent-Type: text/html\r\nContent-Length: 0\r\n\r\n"
                    );
                } else {
                    response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
                }

                _stream.write(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
