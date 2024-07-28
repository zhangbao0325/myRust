#[tokio::main]


async fn main() ->Result<()> {
    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![ops::fetch::init()],
        ..Default::default()
    };

    let mut rt = JsRuntime::new(options);
    
    let js_file = format!("{}/src/fetch.js", env!("CARGO_MANIFEST_DIR"));
    execute_main_module(&mut rt, js_file).await?;
    Ok(())
    
}