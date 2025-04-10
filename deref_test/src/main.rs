struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn hello(name: &str) {
    println!("Hello, {}!", name);
}



fn main() {
    let x = MyBox::new(String::from("rust"));
    assert_eq!("rust", *x);

    // hello(x); // error
    hello(&x);  // ok，传递MyBox<T>的引用， 函数调用时触发Deref trait
    println!("Hello, world!");
}
