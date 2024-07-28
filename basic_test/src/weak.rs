use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    next: Option<Weak<RefCell<Node>>>,
}

#[derive(Debug,Clone)]
pub struct Temp {
    value : RefCell<bool>
}

fn main() {
    let first = Rc::new(RefCell::new(Node {
        value: 1,
        next: None,
    }));

    let second = Rc::new(RefCell::new(Node {
        value: 2,
        next: Some(Rc::downgrade(&first)),
    }));


    if let Some( ref weak_ref) = second.borrow().next {
        if let Some(ref strong_ref) = weak_ref.upgrade() {
            first.borrow_mut().next = Some(Rc::downgrade(&strong_ref));
        }
    }

    // 打印节点的值
    println!("第一个节点的值: {}", first.borrow().value);
    let first1 = first.borrow();
    if let Some(ref weak_ref) = first1.next {
        if let Some(ref strong_ref) = weak_ref.upgrade() {
            println!("第二个节点的值: {}", strong_ref.borrow().value);
        }
    }

    // let mystr = String::from("hello world");
    // let var1 = RefCell::new(mystr);
    // let mut var2 = var1.borrow_mut();

    // *var2 = String::from("hello world2");
    // let var3 = var1.borrow().clone();
    // println!("{:?} {:?} {:?}", var1, var2, var3);

    // let data = RefCell::new(5);

    // // 获取RefCell的可变引用
    // let mut borrow_mut = data.borrow_mut();

    // // 通过replace方法获取原始值的所有权
    // let original_value = data.replace(10);

    // // 打印原始值
    // println!("Original value: {}", original_value);



    let mut temp1 = Temp {
        value: RefCell::new(false),
    };

    let temp2 = temp1.clone();

    let mut v = temp2.value.borrow_mut();
    *v = true;
    

    println!("{:?}", temp1.value);

}