use itertools::Itertools;

fn main() {
    let err_str = "some error";
    let input = vec![Ok(1), Err(err_str), Ok(7)];

    // 写法1： 通过调用cloned()方法，对向量里的每个值进行clone。
    // let it = input
    //     .iter()
    //     .cloned()
    //     .filter_map_ok(|i| if i > 1 { Some(i * 2) } else { None });

    let it = input
        .into_iter()
        .filter_map_ok(|i| if i > 1 { Some(i * 2) } else { None });

    // into_iter会转移input的所有权， 此时 input 不再有效，以下代码将导致编译错误
    // println!("{:?}", input); // 这行代码会报错
    let res = it.collect::<Vec<Result<i32, &str>>>();
    println!("result: {:?}", res);
}
