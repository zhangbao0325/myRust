use std::io::Result;
// fn main() -> Result<()> {
//     tonic_build


//     prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
//     Ok(())
// }


// 为什么没有生成对应的items.rs文件？ 也没有报错，很奇怪。
fn main() -> Result<()> {
    tonic_build::configure()
        .build_server(false) // 如果只需要生成客户端代码，可以将此行注释掉
        .compile(&["src/items.proto"], &["src/"])
        .unwrap();

        Ok(())
}