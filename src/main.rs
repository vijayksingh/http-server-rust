use std::net::TcpListener;

fn main() {
    //  Binding the listener to the (host and port)
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    // Listening for incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
