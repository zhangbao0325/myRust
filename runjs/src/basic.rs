use std::{rc::Rc, path::PathBuf};


use deno_core::{JsRuntime, RuntimeOptions, FsModuleLoader, resolve_url_or_path};
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()> {
    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);
    let path = format!("{}/src/basic.js", env!("CARGO_MANIFEST_DIR"));
    let current_dir = std::env::current_dir().unwrap();
    execute_main_module(&mut rt, &path, &current_dir).await?;
 
    
    // let code = include_str!("basic.js");
    // rt.execute_script(code).await?;

    Ok(())
}