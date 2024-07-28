use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}, thread::{sleep, self}, time::Duration,
};

use web_server::ThreadPool;


fn main() {

    let thread_pool = ThreadPool::new(10);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap();
        println!("Connection established: {:?}", stream);

        thread_pool.execute( || {
            handle_connection(stream)
        });
    }
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

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



    let content = fs::read_to_string(format!("./src/{}", file)).unwrap();
    let content_length = content.len();

    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status_line, content_length, content
    );
    stream.write_all(response.as_bytes()).unwrap();

}


#[allow(dead_code)]
fn handle_connection2(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|x| x.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Connection established: {:#?}", http_request);

    // let response = "HTTP/1.1 200 OK\r\nokkkk\r\n";

    let status_line = "HTTP/1.1 200 OK";

    let content = fs::read_to_string("./src/hello.html").unwrap();
    let content_length = content.len();

    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status_line, content_length, content
    );

    stream.write_all(response.as_bytes()).unwrap();
}
