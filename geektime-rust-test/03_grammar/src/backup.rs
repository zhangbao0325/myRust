
use crate::List::{Cons, Nil};

/*******************************/
fn main1() {
    let s = String::from("hello world");
    take_ownership(s);
    // println!("{}", s);    // 此处报错

    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

/*******************************/

// enum List {
//     Cons(i32, List),
//     Nil,
// }



// fn main() {
//     let list = Cons(1, Cons(2, Cons(3, Nil)));
//     println!("{:?}", list);
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn main2() {
    let list = Cons(1, 
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    // println!("{:?}", list.0);
}


/**********Deref && Drop*********************/
fn main3() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn main4() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}


struct Test1 {
    data: String
}

impl Drop for Test1 {
    fn drop(&mut self) {
        println!("Dropping Test1 with data {}", self.data);
    }
}

fn main5() {
    let t1 = Test1 { data: String::from("t1") };
    let t2 = Test1 { data: String::from("t2") };
    println!("Test1 Created!")
}


fn main6() {
    let t1 = Test1 { data: String::from("t1") };
    let t2 = Test1 { data: String::from("t2") };
    //t1.drop();   // 不允许
    drop(t1);     // 允许
    println!("Test1 Created!")
}


/*************Rc<T>************/
// fn main7() {
//     let a = Cons(5, 
//         Box::new(Cons(10,
//             Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a));
// }

use std::rc::Rc;

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

fn main7() {
    let a = Rc::new(List2::Cons(5, Rc::new(List2::Cons(10, Rc::new(List2::Nil)))));
    // 1
    // let b = List2::Cons(3, Rc::clone(&a));
    // let c = List2::Cons(4, Rc::clone(&a));

    // 2
    let b = List2::Cons(3, a.clone());
    let b = List2::Cons(4, a.clone());
}




/**********Arc **********/
struct Hello {
    v: i32,
}

impl Hello {
    fn say_hello(&self) {
        println!("Hello, world! {}", self.v);
    }

    fn say_id(&self) {
        // println!("id = {:?}",thread::current().id());
        println!("id = {:?}, self = {:p}, self.v = {} ",thread::current().id(), self, self.v);
    }

    fn change(&mut self) {
        self.v += 10;
    }
}

// 直接传值，转移所有权
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main8() {
    let h = Hello{v:10};

    h.say_hello();

    thread::spawn(move || {
        h.say_hello();
        // h.v = 20;     // 不可变借用，无法改变
        println!("h.v: {}", h.v);
    });

    // h.v = 30;    // 所有权已经转移到子线程，主线程无法再使用
    // println!("h.v: {}", h.v);
}

// 传递引用， 借用。
// fn main9() {
//     let h = Hello{v:10};
//     let r_h = &h;   

//     thread::spawn(move || {
//         r_h.say_hello();     // 引用指向的对象生命周期有可能不够长
//     });

//     h.say_hello();
// }

// 使用Arc指针
fn main10() {
    let h = Arc::new(Hello{v:10});

    thread::spawn(move || {
        h.say_hello();
    });

    thread::sleep(Duration::from_secs(1));
}

// Arc只实现了Deref， 没实现defremut
fn main11() {
    let h = Arc::new(Hello{v:10});

    for _ in 0..10 {
        let h = h.clone();
        thread::spawn(move || {
            // h.say_hello();
            h.say_id();

        });

    }

    thread::sleep(Duration::from_secs(1));
}

// 通过Mutex实现多线程读写
fn main12() {
    let h = Arc::new(Mutex::new(Hello{v:10}));

    for _ in 0..10 {
        let h = h.clone();  // Mutex实现了Deref和DerefMut
        thread::spawn(move || {
            let mut l = h.lock().unwrap();
            l.change();
            l.say_id();
        });  // 作用域结束后自动unlock，释放锁

    }

    thread::sleep(Duration::from_secs(1));
}







// #![feature(negative_impls)]

// #[derive(Debug)]
// struct Foo {}
// impl !Send for Foo {}

// fn main() {
//     let foo = Arc::new(Foo {});
//     spawn(move || {
//         dbg!(foo);
//     });
// }