use std::{path::Path, vec};
use std::env;

use deno_core::{JsRuntime, RuntimeOptions, anyhow::Result, anyhow::Error, futures::future::ok, snapshot_util, };

const SNAPSHOT_PATH : &str = "snapshots/main.bin";


fn main() -> Result<()>{
    // let options = RuntimeOptions{
    //     ..Default::default()
    // };
    // let mut rt = JsRuntime::new(options);


    std::env::set_var("CARGO_MAINIFEST_DIR", "/Users/mochen/Documents/mytest/rust/myexamples");
    let dir = std::env::var("CARGO_MAINIFEST_DIR")?;
    let filename = Path::new(&dir).join(SNAPSHOT_PATH);
    let create_snapshot_options = snapshot_util::CreateSnapshotOptions {
        cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
        snapshot_path: filename,
        startup_snapshot: None,
        extensions:vec![],
        compression_cb: None,
        with_runtime_cb: None,
        snapshot_module_load_cb: None,


    };
    snapshot_util::create_snapshot(create_snapshot_options);
    // let data = rt.snapshot();
    Ok(())

}