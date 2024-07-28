use std::{
    fs,
    io::{BufRead, BufReader, Write},
    thread::{sleep, self}, time::Duration, rc::Rc,
};


use tokio::{sync::mpsc, net::{TcpStream,TcpListener}};

use web_server::ThreadPool;


#[tokio::main]
async fn main() {

    let thread_pool = ThreadPool::new(10);

    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        println!("Connection established: {:?}", stream);

        let sender = thread_pool.sender.clone();    
        sender.unwrap().send(stream).await.unwrap();
    }


    println!("Hello, world!");
}



// #[allow(dead_code)]
// fn handle_connection2(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let http_request: Vec<_> = buf_reader
//         .lines() 
//         .map(|x| x.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();

//     println!("Connection established: {:#?}", http_request);

//     // let response = "HTTP/1.1 200 OK\r\nokkkk\r\n";

//     let status_line = "HTTP/1.1 200 OK";

//     let content = fs::read_to_string("./src/hello.html").unwrap();
//     let content_length = content.len();

//     let response = format!(
//         "{}\r\nContent-Length:{}\r\n\r\n{}",
//         status_line, content_length, content
//     );

//     stream.write_all(response.as_bytes()).unwrap();
// }
