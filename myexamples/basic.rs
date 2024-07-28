use std::{rc::Rc};

use deno_core::{anyhow::Result, JsRuntime, Snapshot};
use myexamples::execute_main_module;


static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!( "/Users/mochen/Documents/mytest/rust/myexamples", "/snapshots/main.bin"));

#[tokio::main]
async fn main() -> Result<()>{
    let options = deno_core::RuntimeOptions{
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        startup_snapshot: Some(Snapshot::Static(RUNTIME_SNAPSHOT)),
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);
    let path_str = "./basic_module.js";
    let binding = std::env::current_dir().unwrap();
    let current_dir = binding.as_path();

    execute_main_module(&mut rt, path_str, current_dir).await?;
    Ok(())
}


