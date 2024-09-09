fn main() {
    prost_build::compile_protos(&["./src/greet.proto"], &["./"]).unwrap();
}

//OUT_DIR="/work/mochen.zb/myRust/prost-test" cargo run
