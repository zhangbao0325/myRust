use std::{net::{SocketAddr, TcpListener}, thread};




fn main() -> std::io::Result<()> {

    
    let mut  threads :Vec<thread::JoinHandle<()>>= vec![];

    for i in 0 .. 3 {
        let thread = thread::spawn(|| {
            let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
            let listener = TcpListener::bind(&addr).unwrap();

            for stream in listener.incoming() {
                println!("hello");
            }
        });

        threads.push(thread);   
    }


    for t in threads {
        t.join();
    }

    
    print!("aaaaa");

    Ok(())
}