use std::{rc::Rc, cell::RefCell, borrow::Cow, str::FromStr, default};

use deno_core::{anyhow, error::AnyError, OpState, op, ByteString, Extension, Op, ExtensionFileSource, ExtensionFileSourceCode, OpDecl, anyhow::Ok};
use reqwest::{Request, header::{HeaderName, HeaderValue}, Method, Url};
use serde::{Serialize, Deserialize};


// deno_core::extension!(
//     my_fetch,
//     state = init(),
// );

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchArgs {
    method: String,
    url: String,
    headers: Vec<(ByteString, ByteString)>,
    body: Option<Vec<u8>>,
}

#[derive(Serialize)]
pub struct FetchResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<(ByteString, ByteString)>,
    pub body: Option<Vec<u8>>,
}

#[op]
async fn op_fetch(state: Rc<RefCell<OpState>>, args: FetchArgs) -> Result<FetchResponse, AnyError> {
    let state_ref = state.borrow();
    let client = state_ref.borrow::<reqwest::Client>().clone();
    let url = Url::parse(&args.url)?;
    let method = Method::from_str(&args.method.to_ascii_uppercase())?;
    let mut req = client.request(method, url);
    for (k,v) in args.headers {
        req = req.header(HeaderName::from_bytes(&k)?,HeaderValue::from_bytes(&v)?);
    }

    let req  = if let Some(body) = args.body {
        // req.body(Vec::from(&*body))
        req.body(body)
    } else {
        req
    };
 
    let res = req.send().await?;
    let headers = res.headers().iter().map(|(k,v)| (k.as_str().into(), v.as_bytes().into())).collect();
    let status = res.status().as_u16();
    let status_text = res.status().to_string();
    let body = res.bytes().await?;
    let body =  if body.is_empty() {
        None
    } else {
        // Some(body.into())
        Some(body.to_vec())
    };

    Ok(FetchResponse {
        status,
        status_text,
        headers,
        body,
    })
}


#[warn(deprecated)]
pub fn init() ->Extension {
    let js_file: Cow<'static, [ExtensionFileSource]> = std::borrow::Cow::Borrowed(&[ExtensionFileSource {
        specifier: "src/ops/fetch.js",
        // code: Some(ExtensionFileSourceCode::Text("console.log('Hello, Deno!');".to_string())),
        code: ExtensionFileSourceCode::IncludedInBinary(
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/ops/", "fetch.js"))),
    }]);

    let ops : Cow<'static,[OpDecl]> = std::borrow::Cow::Borrowed(&[op_fetch::DECL]);
    
    

    // Extension::builder("my_fetch").ops(
    //     vec![op_fetch::DECL]
    // ).build();
    print!("js_file: {:?}", js_file);

    Extension {
        name: "fetch",
        js_files: js_file, 
        ops: ops,
        ..Default::default()       
    }
}