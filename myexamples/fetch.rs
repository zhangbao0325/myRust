use std::rc::Rc;

use deno_core::{anyhow::Result, RuntimeOptions, FsModuleLoader,  JsRuntime};

use myexamples::execute_main_module;
use myexamples::ops::fetch;

// use deno_core_live::{execute_main_module, ops};


#[tokio::main]
async fn main() ->Result<()> {
    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![fetch::init()],
        ..Default::default()
    };

    let mut rt = JsRuntime::new(options);
    // rt.register(init);

    // let js_file = format!("{}/fetch.js", env!("CARGO_MANIFEST_DIR"));
    let path_str = "./fetch.js";
    let binding = std::env::current_dir().unwrap();
    let current_dir = binding.as_path();

    execute_main_module(&mut rt, path_str, current_dir).await?;
    Ok(())
    
}