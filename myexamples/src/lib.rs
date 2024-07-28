use std::path::{PathBuf, Path};

use deno_core::{anyhow::Result, JsRuntime, RuntimeOptions, v8, serde_v8, resolve_path, op2};
use serde::de::DeserializeOwned;
use std::{rc::Rc, cell::RefCell, borrow::Cow, str::FromStr, default};

use deno_core::{anyhow, error::AnyError, OpState, op, ByteString, Extension, Op, ExtensionFileSource, ExtensionFileSourceCode, OpDecl, anyhow::Ok};
use reqwest::{Request, header::{HeaderName, HeaderValue}, Method, Url};
use serde::{Serialize, Deserialize};


pub mod ops;


#[warn(dead_code, unused_mut)]
pub async fn test01() -> Result<()>{
    let mut options = RuntimeOptions::default();
    let mut rt = JsRuntime::new(options);
    let name: &str = "mochen";
    let code = include_str!("../basic.js");
    let result: String= eval(name, code, &mut rt).await?;
    
    println!("{:?}", result);
    Ok(())
}

pub async fn eval<T>(name: &'static str, code : &'static str, rt: &mut JsRuntime) ->Result<T> 
where T: DeserializeOwned, 
{
    let ret = rt.execute_script_static(name, code)?;
    let result = rt.resolve_value(ret).await?;
    let scope = &mut rt.handle_scope();
    let result  = v8::Local::new(scope, result);
    Ok(serde_v8::from_v8(scope, result)?)
}


#[allow(dead_code)]
pub async fn execute_main_module(rt: &mut JsRuntime, path_str: &str, current_dir: &Path) ->Result<()> {

    let mod_specifier =  resolve_path(path_str, current_dir)?;
    let id = rt.load_main_module(&mod_specifier, None).await?;
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