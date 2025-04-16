use futures::future::{self, Either, FutureExt};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // 定义两个异步任务
    let task1 = async {
        sleep(Duration::from_secs(2)).await; // 模拟耗时任务
        "Task 1 completed"
    };

    let task2 = async {
        sleep(Duration::from_secs(1)).await; // 模拟耗时任务
        "Task 2 completed"
    };

    // 使用 future::select 竞速两个任务
    match future::select(task1.boxed(), task2.boxed()).await {
        Either::Left((result, _)) => {
            // task1 先完成
            println!("{}", result);
        }
        Either::Right((result, _)) => {
            // task2 先完成
            println!("{}", result);
        }
    }

}
