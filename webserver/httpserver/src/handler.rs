use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, env, fs};

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn file_load(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let file_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(file_path);
        contents.ok()
    }
}

pub struct PageNotFoundHandler;
pub struct StaticPageHandler;
pub struct WebServiceHandler;

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        // HttpResponse::new("404", Some(vec![("Content-Type", "text/plain")]), Some("Page Not Found".to_string()))
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "text/plain");
        HttpResponse::new("404", Some(headers), Self::file_load("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource; //TODO: 这是什么写法
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::file_load("index.html")),
            "health" => HttpResponse::new("200", None, Self::file_load("health.html")),
            path => match Self::file_load(path) {
                Some(contents) => {
                    let mut headers = HashMap::new();
                    if path.ends_with(".css") {
                        headers.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        headers.insert("Content-Type", "text/javascript");
                    } else {
                        headers.insert("Content-Type", "text/html");
                    }

                    HttpResponse::new("200", Some(headers), Some(contents))
                }
                _ => HttpResponse::new("404", None, Self::file_load("404.html")),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contents = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[2] {
            "shipping" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            _ => HttpResponse::new("404", None, Some("Not Found".to_string())),
        }
    }
}
