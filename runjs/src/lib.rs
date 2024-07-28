use std::path::PathBuf;

use deno_core::{anyhow::Result, JsRuntime, resolve_url_or_path};
pub mod ops;





#[allow(dead_code)]
pub async fn execute_main_module(rt: &mut JsRuntime, path: impl AsRef<str>, current_dir: &PathBuf) -> Result<()> {
    let url  = resolve_url_or_path(path.as_ref(), &current_dir).unwrap();
    let id = rt.load_main_module(&url, None).await.unwrap();

    let mut receiver = rt.mod_evaluate(id);
    tokio::select! {
        resolved = &mut receiver => {
            resolved.expect("failed to evaluate module")
        }
        _ = rt.run_event_loop(false) => {
            receiver.await.expect("failed to evaluate module")
        }
    }
}