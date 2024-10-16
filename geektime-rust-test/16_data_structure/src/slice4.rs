use std::fmt;
fn main() {
    let s = String::from("hello");
    print_slice(&s);
    print_slice(&s[..]);

    // String 支持 AsRef<str>
    print_slice1(&s);
    print_slice1(&s[..]);
    print_slice1(s.clone());

    // String 也实现了 AsRef<[u8]>，所以下面的代码成立
    // 打印出来是 [104, 101, 108, 108, 111]
    print_slice2(&s);
    print_slice2(&s[..]);
    print_slice2(s);

    let s = String::from("hello");
    let slice1: &str = s.as_ref(); // 这里不会拿走 s 的所有权
    let slice2: &[u8] = s.as_ref(); // 这里不会拿走 s 的所有权
    println!("{:?}", slice1); // 打印出str切片
    println!("{:?}", slice2); // 打印出[u8]切片
    println!("{:?}", s); // s仍然可以使用
}

fn print_slice(s: &str) {
    println!("{:?}", s);
}

fn print_slice1<T: AsRef<str>>(s: T) {
    println!("{:?}", s.as_ref());
}

fn print_slice2<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}

#[allow(dead_code)]
fn print_slice3<T, U>(s: T)
where
    T: AsRef<U>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}
