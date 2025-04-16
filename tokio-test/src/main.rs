use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
    task, time,
};

async fn mock_run_event_loop() {
    println!("mock_run_event_loop1");
    test().await;
    println!("mock_run_event_loop2");
}


async fn test() {
    loop {

        // println!("test");
    }
}

async fn mock_checker_run() {
    println!("mock_checker_run1");
    
    // test().await;
    println!("mock_checker_run2");
}


fn main() {

        let (tx, mut rx) = mpsc::channel::<u32>(2);
    // let worker_thread = std::thread::Builder::new().name("worker1".to_string()).spawn(move || {

    //     let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    //     rt.block_on(async move {
    //         loop {
    //             println!("worker start loop...");
    //             tokio::select! {
    //                 _ = rx.recv() => {
    //                     println!("worker thread received message");
    //                 }

    //                 _ = futures::future::pending::<()>() => {
    //                     println!("worker thread is waiting");
    //                 }

    //                 res = mock_run_event_loop() => {

    //                 }
    //             }
    //         }
    //     });

    //     println!("worker thread started");
    // }).unwrap();

    
    let main_fut = async {

        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

        let mut cron = time::interval(std::time::Duration::from_secs(1));

        // task::spawn_local(async {
        //     println!("init checker1");
            
        //     mock_checker_run();
        //     println!("init checker2");
        // });

        loop {
            println!("main start loop...");
            tokio::select! {
                incoming = listener.accept() => {
                    println!("accepted socket1");
                    let (socket, _) = incoming.unwrap();
                    println!("accepted socket2");
                }

                _ = cron.tick() => {
                    println!("cron ticked1");
                    tx.send(1).await.unwrap();
                    println!("cron ticked2");
                }

                _ = futures::future::pending::<()>() => {
                    println!("main thread is waiting");
                }
            }
        }

        // loop {
            
        // }
    };

    let local_set = task::LocalSet::new();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(local_set.run_until(main_fut));
}
