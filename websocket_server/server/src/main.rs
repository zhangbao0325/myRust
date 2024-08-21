use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures::{
    channel::mpsc::{self, UnboundedSender},
    future, StreamExt, TryStreamExt,
};
use tokio::sync::broadcast;
// use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>>;

const SERVER_ADDR: &str = "127.0.0.1:8080";

// 问题1： listener accept 和listener incoming的区别是？
// 问题2： mpsc::unbounded_channel() 和 mpsc::unbounded()  ,前者是tokio的，后者是future的
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind(SERVER_ADDR).await.unwrap();

    let peers: PeerMap = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, peer_addr)) = listener.accept().await {
        let peers = Arc::clone(&peers);
        tokio::spawn(handle_connection(stream, peers, peer_addr));
    }

    println!("Hello, world!");
}

async fn handle_connection(
    tcp_stream: tokio::net::TcpStream,
    peers: PeerMap,
    peer_addr: SocketAddr,
) {
    let ws_stream = tokio_tungstenite::accept_async(tcp_stream)
        .await
        .expect("ws握手失败!");
    println!("成功建立ws连接: {}", peer_addr);

    let (write, read) = ws_stream.split();

    let (msg_sender, msg_receiver) = mpsc::unbounded();

    peers.lock().unwrap().insert(peer_addr, msg_sender);

    let receive_and_broadcast_message = read.try_for_each(|message| {
        match message.clone() {
            Message::Text(text_message) => {
                println!("{}: {}", peer_addr, text_message);
                let peers = peers.lock().unwrap();
                let broadcast_peers = peers.iter().filter(|(addr, _)| addr != &&peer_addr);

                for (broadcast_addr, broadcast_sender) in broadcast_peers {
                    if !broadcast_sender.is_closed() {
                        if let Err(err) = broadcast_sender.unbounded_send(message.clone()) {
                            eprintln!("无法广播消息:{:?}:{}", err, broadcast_addr);
                        }
                    }
                }
            }

            Message::Close(_) => {
                peers.lock().unwrap().remove(&peer_addr);
                print!("{} 断开了连接", peer_addr);
            }

            _ => {
                eprintln!("不支持的数据类型");
            }
        }

        future::ok(())
    });

    let forward_message = msg_receiver.map(Ok).forward(write);

    if let Err(err) = tokio::try_join!(receive_and_broadcast_message, forward_message) {
        eprintln!("广播消息时发生错误: {:?}", err);

        peers.lock().unwrap().remove(&peer_addr);
        println!("{}断开了连接", peer_addr);
    };
}
