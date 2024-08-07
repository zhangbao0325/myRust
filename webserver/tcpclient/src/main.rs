use std::{io::{Read, Write}, net::TcpStream};




fn main() {
    let mut client = TcpStream::connect("127.0.0.1:3000").unwrap();
    client.write(b"Hello, Server!\n").unwrap();
    
    let mut buffer = [0;1024];
    client.read(&mut buffer).unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buffer[..]));

}
