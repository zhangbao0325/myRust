//cargo run -p closure --example fn_mut1
fn main() {
    let mut name = String::from("zhangbao");
    let mut name1 = String::from("menggesi");

    let mut c = || {
        name.push_str(": ningmeng");
        println!("{}", name);
    };

    let mut c1 = move || {
        name1.push_str(": ningmeng");
        println!("{}", name1);
    };

    c();
    c1();

    call_mut(&mut c);
    call_mut(&mut c1);

    call_once(c);
    call_once(c1);
}

fn call_mut(mut c: impl FnMut()) {
    c();
}

fn call_once(c: impl FnOnce()) {
    c();
}
