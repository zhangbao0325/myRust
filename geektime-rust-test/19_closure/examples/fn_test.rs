fn main() {
    let mut name = String::from("mochen");
    let vec = vec!["zhangbao", "menggesi", "lenmon"];
    let vec1 = &vec[..];
    let data = (1, 2, 3, 4);

    let c = move || {
        println!("data: {:?}", data);
        println!("vec1: {:?}, name: {:?}", vec1, name.clone());
    };

    c();
    // println!("c len is {:?}", c.len());
    println!("vec len is {:?}", vec1);
    println!("data len is {:?}", data);

    // 因为 name 被 move 了，所以以下代码会出错
    // println!("name is {:?}", name);
    // name.push_str("xiaoningmeng");
}
