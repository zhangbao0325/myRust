// build.rs

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {


     // Tell cargo to rerun this script if the C source file changes
     println!("cargo:rerun-if-changed=src/c_utils.c");

     // Get the directory where build artifacts are stored
     let out_dir = env::var("OUT_DIR").expect("No out dir");
 
     // Compile the C source file into an object file
     let c_file = "src/c_utils.c";
     let output = format!("{}/c_utils.o", out_dir);
     Command::new("cc")
         .args(&[c_file, "-c", "-o"])
         .arg(&output)
         .status()
         .expect("Failed to compile C file");
 
    // After compiling c_utils.o
    Command::new("ar")
    .args(&["crus", "libc_utils.a", "c_utils.o"])
    .current_dir(&Path::new(&out_dir))
    .status()
    .expect("Failed to archive object file into static library");  

    // 将输出目录添加到链接器的搜索路径中
    println!("cargo:rustc-link-search=native={}", out_dir);    
    println!("cargo:rustc-link-lib=static=c_utils");


    // 告诉 cargo 链接 c_utils 库
    // println!("cargo:rustc-link-lib=static=c_utils");
}
