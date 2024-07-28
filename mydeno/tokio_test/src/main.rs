use std::rc::Rc;
use tokio::task;

#[tokio::main]
async fn main() {
    let nonsend_data = Rc::new("my nonsend data...");

    let local = task::LocalSet::new();

    // Run the local task set.
    local.run_until(async move {
        let nonsend_data = nonsend_data.clone();
        task::spawn_local(async move {
            println!("{}", nonsend_data);
            // ...
        }).await.unwrap();
    }).await;
}