use std::{
    sync::{ Arc}, thread, time::Duration,
};

use tokio::{sync::Mutex, io::{BufReader, AsyncBufReadExt, AsyncWriteExt}};

use tokio::{sync::mpsc, net::TcpStream, runtime::Builder, task::{JoinHandle, self}};

pub struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next_line().await.unwrap().unwrap();
    
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
        stream.write_all(response.as_bytes()).await;
    
    
        Ok(())
    
    }


    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<TcpStream>>>) -> Worker {
            let thread = tokio::spawn(async move {
                loop {
                    let stream = {
                        let mut rx = rx.lock().await;
                        rx.recv().await.unwrap()
                    };

                    println!("worker {} handle the connection", id);
                    Self::handle_connection(stream).await;
                }
            });
        Worker {
            id,
            thread: Some(thread),
        }
    }

}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: Option<mpsc::Sender<TcpStream>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<TcpStream>(100);
        
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }


    pub async fn execute(self, stream: TcpStream)
    {
        self.sender.unwrap().send(stream).await.unwrap();
    }


    // pub async fn execute<F>(&self, f: F)
    // where
    //     F: FnOnce() + Send + 'static,
    // {
    //     let job = Box::new(f);
    //     self.sender.as_ref().unwrap().send(job).await;
    // }


    // pub async fn handle(&self, stream: TcpStream)
    // // where
    // {
    //     self.sender.as_ref().unwrap().send(stream).await;
    // }


}

// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         drop(self.sender.take());
//         for worker in &mut self.workers {
//             println!("Shutting down worker {}", worker.id);
//             if let Some(thread) = worker.thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }
