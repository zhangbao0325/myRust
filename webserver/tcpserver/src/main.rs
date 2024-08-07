use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();

                let message = String::from_utf8_lossy(&buffer[..]);
                println!("Received: {}", message);

                stream.write(b"HTTP/1.1 200 OK\r\n\r\nHello, World!\n").unwrap();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
