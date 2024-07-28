use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server running on 127.0.0.1:8080");

    let (tx, mut rx) = mpsc::channel::<TcpStream>(100);
    let rx = Arc::new(Mutex::new(rx));

    for _ in 0..4 {
        let rx = Arc::clone(&rx);
        let tx = tx.clone();
        tokio::spawn(async move {
            loop {
                let stream = {
                    let mut rx = rx.lock().await;
                    rx.recv().await.unwrap()
                };
                handle_connection(stream).await.unwrap();
            }
        });
    }

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tx.send(socket).await.unwrap();
    }
}

async fn handle_connection(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // let mut buffer = [0; 1024];
    // socket.read(&mut buffer).await?;

    // let response = b"HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nhello world";
    // socket.write_all(response).await?;

    let buf_reader = BufReader::new(&mut socket);
    let request_line = buf_reader.lines().next_line().await.unwrap().unwrap();

    // let (status_line, file) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // }; 

    let (status_line, file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };



    let content = "abcd";
    let content_length = content.len();

    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status_line, content_length, content
    );
    socket.write_all(response.as_bytes()).await;


    Ok(())





}
