use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::{Context, Poll},
        time::Duration,
    },
    // 引入之前实现的定时器模块
    timer_future::TimerFuture,
};

/// 任务执行器，负责从通道中接收任务然后执行
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner`负责创建新的`Future`然后将它发送到任务通道中
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// 一个Future，它可以调度自己(将自己放入任务通道中)，然后等待执行器去`poll`
struct Task {
    /// 进行中的Future，在未来的某个时间点会被完成
    ///
    /// 按理来说`Mutex`在这里是多余的，因为我们只有一个线程来执行任务。但是由于
    /// Rust并不聪明，它无法知道`Future`只会在一个线程内被修改，并不会被跨线程修改。因此
    /// 我们需要使用`Mutex`来满足这个笨笨的编译器对线程安全的执着。
    ///
    /// 如果是生产级的执行器实现，不会使用`Mutex`，因为会带来性能上的开销，取而代之的是使用`UnsafeCell`
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// 可以将该任务自身放回到任务通道中，等待执行器的poll
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // 任务通道允许的最大缓冲数(任务队列的最大长度)
    // 当前的实现仅仅是为了简单，在实际的执行中，并不会这么使用
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("任务队列已满");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 通过发送任务到任务管道的方式来实现`wake`，这样`wake`后，任务就能被执行器`poll`
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("任务队列已满");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于任务自身创建一个 `LocalWaker`
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>`是`Pin<Box<dyn Future<Output = T> + Send + 'static>>`的类型别名
                // 通过调用`as_mut`方法，可以将上面的类型转换成`Pin<&mut dyn Future + Send + 'static>`
                if future.as_mut().poll(context).is_pending() {
                    // Future还没执行完，因此将它放回任务中，等待下次被poll
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // 生成一个任务
    spawner.spawn(async {
        println!("howdy!");
        // 创建定时器Future，并等待它完成
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);

    // 运行执行器直到任务队列为空
    // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
    executor.run();
}



/*
我看代码里还有一处这样写：
let future = future.boxed(); 将future转化成了 std::pin::Pin<Box<dyn Future<Output = ()> + Send>>， 这是为什么要这么做？

Monica
2/2
将 future 转化为 std::pin::Pin<Box<dyn Future<Output = ()> + Send>> 的原因主要与 Rust 的异步编程模型和内存安全性相关。下面是几个关键点，解释为什么要进行这种转换：

1. 动态分发
dyn Future: 将 Future 转换为 dyn Future 允许我们使用动态分发。这意味着我们可以处理不同类型的 Future，而不需要在编译时知道具体的类型。这在需要将不同的异步任务存储在同一个容器中时非常有用。
2. 堆分配
Box: 使用 Box 将 Future 放在堆上，而不是栈上。由于异步任务可能会有不同的大小，使用 Box 可以确保我们能够在运行时动态分配内存。
3. Pinning 的要求
Pin: 在 Rust 中，某些类型（如 Future）在异步上下文中要求被“固定”。这意味着一旦它们被放置在内存中，就不能被移动。这是因为移动可能会导致内部状态的无效性。使用 Pin<Box<T>> 确保了 Future 在其生命周期内不会被移动，从而保证了内存安全。
4. Send Trait
Send: 在多线程环境中，确保 Future 可以安全地在不同线程之间传递是很重要的。通过将 Future 转换为 dyn Future<Output = ()> + Send，我们确保了这个 Future 可以安全地在线程间传递。
代码示例
假设我们有一个 Future 类型的任务，转换过程可能如下：

rust
复制
let future: Box<dyn Future<Output = ()> + Send> = some_future.boxed();
这里 some_future 是某种实现了 Future trait 的类型。
boxed() 方法会将其转换为 Box<dyn Future<Output = ()> + Send>，然后再通过 Pin 进行固定。
总结
将 future 转换为 std::pin::Pin<Box<dyn Future<Output = ()> + Send>> 是为了实现以下目标：

支持动态分发，以便处理不同类型的 Future。
在堆上分配内存，适应不同大小的 Future 类型。
确保 Future 在其生命周期内不会被移动，从而保持内存安全。
使 Future 可以安全地在多线程环境中使用。
这样的转换在 Rust 的异步编程中是一个常见且重要的模式。


*/


/*
在指定某个数据结构满足某个条件的时候，有时候用impl，有是有用dyn关键字，比如刚才的代码里用到了fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {}， 这两者有什么区别，分别在什么场合使用？

Monica
在 Rust 中，impl Trait 和 dyn Trait 都用于处理 trait 对象，但它们之间有一些重要的区别和适用场合。以下是对这两者的详细解释：

1. impl Trait
定义: impl Trait 是一种语法，用于表示函数参数或返回值的具体类型是某个 trait 的实现，但具体的类型在编译时是确定的。
优点:
性能: impl Trait 通常在编译时被解析为具体类型，因此可以进行更好的优化，不涉及动态分发。
类型安全: 由于具体类型在编译时已知，编译器可以进行更严格的类型检查。
适用场合:
当你希望函数接受任何实现了某个 trait 的类型，并且不需要在运行时处理多个不同类型时，使用 impl Trait 是合适的。
例如，fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {} 表示 spawn 可以接受任何实现了 Future trait 的类型。
2. dyn Trait
定义: dyn Trait 是用于创建 trait 对象的语法，表示在运行时可以处理多种实现了该 trait 的类型。
优点:
灵活性: 允许在运行时处理不同的类型，这对于需要存储或传递不同类型的 trait 实现时非常有用。
缺点:
性能开销: 使用 dyn Trait 会引入动态分发的开销，因为在运行时需要通过指针查找具体实现。
类型安全: 由于具体类型在运行时才能确定，编译器的类型检查可能不如 impl Trait 严格。
适用场合:
当你需要一个容器来存储多种不同类型的实现，或者需要在运行时决定具体的类型时，使用 dyn Trait 是合适的。
例如，使用 Box<dyn Future<Output = ()> + Send> 来存储不同类型的 Future。
总结
使用 impl Trait:

当你希望在函数签名中指定一个 trait 的实现，并且不需要在运行时处理多个不同的类型时。
性能更好，适合编译时已知类型的场合。
使用 dyn Trait:

当你需要在运行时处理多种不同类型的 trait 实现时。
提供更大的灵活性，但会有性能开销。
例子对比
rust
复制
// 使用 impl Trait
fn spawn_future_impl(future: impl Future<Output = ()> + 'static + Send) {
    // 处理 future
}

// 使用 dyn Trait
fn spawn_future_dyn(future: Box<dyn Future<Output = ()> + Send>) {
    // 处理 future
}
在 spawn_future_impl 中，future 的类型在编译时已知，而在 spawn_future_dyn 中，future 的具体类型在运行时确定。选择哪种方式取决于你对性能和灵活性的需求。
 */