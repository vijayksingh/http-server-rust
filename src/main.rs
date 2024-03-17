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
                let response = format!("HTTP/1.1 200 OK\r\n\r\n");

                _stream.write(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
