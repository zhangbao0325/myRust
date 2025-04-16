fn main() {
    let v = vec![0u8; 1023];
    let v1 = vec![0u8; 1024];

    let mut c = |x: u64| v.len() as u64 * x;

    let mut c1 = move |x: u64| v1.len() as u64 * x;

    println!("directly call: {:?}", c(2));
    println!("directly call: {:?}", c1(2));

    println!("fn call: {:?}", call(2, &c));
    println!("fn call: {:?}", call(2, &c1));

    println!("fnMut call: {:?}", call_mut(2, &mut c));
    println!("fnMut call: {:?}", call_mut(2, &mut c1));

    println!("fnOnce call: {:?}", call_once(2, c));
    println!("fnOnce call: {:?}", call_once(2, c1));
}

fn call(arg: u64, c: impl Fn(u64) -> u64) -> u64 {
    c(arg)
}

fn call_mut(arg: u64, c: &mut impl FnMut(u64) -> u64) -> u64 {
    c(arg)
}

fn call_once(arg: u64, c: impl FnOnce(u64) -> u64) -> u64 {
    c(arg)
}

fn call_once_string(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    c(arg)
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "Rosie".into())
}

fn other_test() {
    let name = String::from("Tyr");

    // 这个闭包会 clone 内部的数据返回，所以它不是 FnOnce
    let c = move |greeting: String| (greeting, name.clone());

    // 所以 c 可以被调用多次
    println!("c call once: {:?}", c("qiao".into()));
    println!("c call twice: {:?}", c("bonjour".into()));

    // ---然而一旦它被当成 FnOnce 被调用，就无法被再次调用----

    // 实际发现并不是，只要这里传引用，下面就可以继续使用 c
    println!("result: {:?}", call_once_string("hi".into(), &c));

    // ---无法再次调用---
    // 这里可以继续使用 c
    let result = c("hi".to_string());
    println!("result2: {:?}", result);

    let result2 = c("hi".to_string());
    println!("result3: {:?}", result2);

    // fn 也可以被当成FnOnce调用，只要接口一致就可以
    println!("result: {:?}", call_once_string("hola".into(), not_closure));
}
