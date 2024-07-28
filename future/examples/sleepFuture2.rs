use std::{time::Duration, future::Future, task::Poll, pin::Pin};

use future::{sleep, SleepFuture};


#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let s = String::from("hello world");

    // foo(v,s).await;

    Foofut::new(v, s).await;

    
}

async fn foo(v: Vec<u8>, s: String)->u32 {
    println!("v: {:?}", v);
    sleep(Duration::from_secs(3)).await;

    println!("s: {}", s);
    sleep(Duration::from_secs(3)).await;
    32
}


struct Foofut {
    state: FoofutState,
    v: Vec<u8>,
    s:String,
}

enum FoofutState {
    Init,
    Sleep1(SleepFuture),
    Sleep2(SleepFuture),
    Done,
}

impl Foofut {
    fn new(v: Vec<u8>, s: String) -> Foofut {
        Self {
            state: FoofutState::Init,
            v: v,
            s: s,
        }
    }
    
}
  
impl Future for Foofut {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        loop {
            match self.as_mut().get_mut().state {
                FoofutState::Init => {
                    println!("Foofut state: {:?}", self.v);
                    let fut1 = SleepFuture::new(Duration::from_secs(3));
                    self.as_mut().get_mut().state = FoofutState::Sleep1(fut1);
                }

                FoofutState::Sleep1(ref mut fut1) => {
                    match Pin::new(fut1).poll(cx) {
                        Poll::Ready(_) => {
                            println!("{}",self.s);
                            let fut2 = SleepFuture::new(Duration::from_secs(3));
                            self.as_mut().get_mut().state = FoofutState::Sleep2(fut2);
                        }
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                    }
                }
                FoofutState::Sleep2(ref mut fut2) => {
                    match Pin::new(fut2).poll(cx) {
                        Poll::Ready(_) => {
                            self.as_mut().get_mut().state = FoofutState::Done;
                            
                        }
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                    }
                }
                FoofutState::Done => {
                    return Poll::Ready(32);
                }
            }
        }
    }

    
    
}