use std::thread;

use anyhow::{Error, Result};
use blake3::Hasher;
use futures::{SinkExt, StreamExt};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
};
use tokio_util::codec::{Framed, LinesCodec};

pub const PREFIX_ZERO: &[u8] = &[0, 0, 0];

#[tokio::main]
async fn main() -> Result<(),Error> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on: {}", addr);

    let (sender, mut receiver) = mpsc::unbounded_channel::<(String, oneshot::Sender<String>)>();

    thread::spawn(move || {
        while let Some((line, reply)) = receiver.blocking_recv() {
            let result = match pow(&line) {
                Some((hash, nonce)) => format!("hash: {}, once: {}", hash, nonce),
                None => "not found".to_string(),
            };

            if let Err(e) = reply.send(result) {
                println!("Failed to send: {}", e);
            }
        }
    });

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted connection from: {}", addr);
        let sender_clone = sender.clone();
        tokio::spawn(async move {
            let framed = Framed::new(stream, LinesCodec::new());
            let (mut w, mut r) = framed.split();

            while let Some(Ok(line)) = r.next().await {
                let (reply, reply_rx) = oneshot::channel();
                sender_clone.send((line, reply))?;

                if let Ok(v) = reply_rx.await {
                    w.send(format!("Pow calculated: {}", v)).await?;
                }
            }

            Ok::<_, anyhow::Error>(())

        });
    }

}

fn pow(s: &str) -> Option<(String, u32)> {
    let hasher = blake3_base_hash(s.as_bytes());
    let nonce = (0..u32::MAX).into_par_iter().find_any(|n| {
        let hash = blake3_hash(hasher.clone(), n).as_bytes().to_vec();
        &hash[..PREFIX_ZERO.len()] == PREFIX_ZERO
    });

    nonce.map(|n| {
        let hash = blake3_hash(hasher, &n).to_hex().to_string();
        (hash, n)
    })
}

fn blake3_hash(mut hasher: blake3::Hasher, nonce: &u32) -> blake3::Hash {
    hasher.update(&nonce.to_be_bytes()[..]);
    hasher.finalize()
}

fn blake3_base_hash(data: &[u8]) -> Hasher {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher
}
