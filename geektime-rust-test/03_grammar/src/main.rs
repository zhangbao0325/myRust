use std::thread::spawn;

#[derive(Debug, Copy, Clone)]
struct User {
    name: i32,
}


fn main() {
    let user = User { name: 10 };

    spawn(|| {
        println!("Hello from the first thread {}", user.name);
    });

    println!("{:?}", user.name)
}