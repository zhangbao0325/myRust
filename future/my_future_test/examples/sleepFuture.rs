use std::{
    future::Future,
    task::{Poll, Waker},
    time::Duration, thread, sync::{Arc, Mutex},
};

#[tokio::main]
async fn main() {
    println!("runtime start... ");
    SleepFuture::new(Duration::from_secs(3)).await;
    println!("runtime end ...");
}

struct SleepFuture {
    duration: Duration,
    state: Arc<Mutex<State>>,
}

struct State {
    waker: Option<Waker>,
    inner_state: InnerState,
}

#[derive(PartialEq)]
enum InnerState {
    Init,
    Sleeping,
    Done,
}

impl SleepFuture {
    fn new(duration: Duration) -> SleepFuture {
        Self {
            duration,
            state: Arc::new(Mutex::new(State {
                waker: None,
                inner_state: InnerState::Init,
            })),
        }
    }
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut guard = self.state.lock().unwrap();

        println!("polling... ");

        if guard.inner_state == InnerState::Done {
            return Poll::Ready(());
        } 

        if guard.inner_state == InnerState::Init {
            guard.waker =  Some(cx.waker().clone());
            guard.inner_state = InnerState::Sleeping;
            
            let state_clone = Arc::clone(&self.state);
            let duration = self.duration;
            thread::spawn(move || {
                println!("sleeping...");
                thread::sleep(duration);
                let mut gurad = state_clone.lock().unwrap();
                gurad.inner_state = InnerState::Done;

                if let Some(waker) = gurad.waker.take() {
                    waker.wake_by_ref();
                }

                println!("weakup...");
            });
        }

        guard.waker = Some(cx.waker().clone());

        Poll::Pending
    }
}
