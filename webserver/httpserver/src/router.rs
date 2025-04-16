use std::io::Write;

use http::httprequest::{self, HttpRequest, Resource};

use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::GET => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
