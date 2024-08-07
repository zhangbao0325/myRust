use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Method {
    GET,
    POST,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::Uninitialized,
        }
    }
}


#[derive(Debug, PartialEq)]
enum Version {
    V1,
    V2,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1,
            "HTTP/2.0" => Version::V2,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}


#[derive(Debug)]
pub struct HttpRequest {
    method: Method,
    version: Version,
    resource: Resource,
    headers: HashMap<String,String>,
    body: Option<String>,
}

fn parse_req_line(line: &str) -> (Method, Version, Resource) {
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap().into();
    let resource = Resource::Path(parts.next().unwrap().to_string());
    let version = parts.next().unwrap().into();

    (method, version, resource)
}

fn parse_header_line(line: &str) -> (String, String) {
    
    let mut parts = line.split(":");
    let key = parts.next().unwrap().trim().to_string();
    let value = parts.next().unwrap().trim().to_string();
    (key,value)

}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut method = Method::Uninitialized;
        let mut version = Version::Uninitialized;
        let mut resource = Resource::Path("".to_string());
        let mut headers = HashMap::new();
        let mut body = None;

        for line in req.lines() {
            if line.contains("HTTP") {
                (method, version, resource) = parse_req_line(line);
            }else if line.contains(":") {
                let (key,value) = parse_header_line(line);
                headers.insert(key, value);
                
            }else if line.is_empty(){
                continue;
            } else {
                body = Some(line.to_string());
            }
        }

        HttpRequest {
            method,
            version,
            resource,
            headers,
            body,
        }

    }
} 
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_from_str() {
        let m : Method = "GET".into();
        let p: Method = "POST".into();
        let u: Method = "UNKNOWN".into();
        assert_eq!(m, Method::GET);
        assert_eq!(p, Method::POST);
        assert_eq!(u, Method::Uninitialized);

    }

    #[test]
    fn test_version_from_str() {
        let v: Version = "HTTP/1.1".into();
        let v2: Version = "HTTP/2.0".into();
        let u: Version = "UNKNOWN".into();
        assert_eq!(v, Version::V1);
        assert_eq!(v2, Version::V2);
        assert_eq!(u, Version::Uninitialized);
    }

    #[test]
    fn test_parse_req_line() {
        let (m, v, r) = parse_req_line("GET /index.html HTTP/1.1");
        assert_eq!(m, Method::GET);
        assert_eq!(v, Version::V1);
        assert_eq!(r, Resource::Path("/index.html".to_string()));
    }

    #[test]
    fn test_parse_header_line() {
        let (k, v) = parse_header_line("Content-Type: text/html");
        assert_eq!(k, "Content-Type".to_string());
        assert_eq!(v, "text/html".to_string());
    }

    #[test]
    fn test_request_from_string() {
        let req_string = String::from("GET /index.html HTTP/1.1\r\nUser-Agent:curl/7.1\r\nHost:mochen.test\r\n\r\n");
        let parsed_req : HttpRequest= req_string.into();
        let mut headers = HashMap::new();
        headers.insert("Host".into(), "mochen.test".into());
        headers.insert("User-Agent".into(), "curl/7.1".into());
        assert_eq!(parsed_req.method, Method::GET);
        assert_eq!(parsed_req.version, Version::V1);
        assert_eq!(parsed_req.resource, Resource::Path("/index.html".to_string()));
        assert_eq!(parsed_req.headers, headers);
    }
}
