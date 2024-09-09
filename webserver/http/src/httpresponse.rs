use std::{collections::HashMap, fmt::Error, io::Write, os::macos::raw::stat};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

// impl<'a> From<String> for HttpResponse<'a> {
//     fn from(res: String) -> Self {

//     }
// }

impl<'a> From<HttpResponse<'a>> for String {
    fn from(response: HttpResponse) -> String {
        format!(
            "{} {} {}\r\n{}\r\n{}",
            response.version,
            response.status_code,
            response.status_text,
            response.headers(),
            response.body()
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code;
        }

        response.headers = match headers {
            Some(_) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };

        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal ServerError",
            _ => "Unknown",
        };

        response.body = body;

        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<(), Error> {
        let res = self.clone();
        let _ = write!(write_stream, "{}", String::from(res));
        Ok(())
    }

    fn headers(&self) -> String {
        let map = self.headers.clone().unwrap(); // point: clone一个option会copy里面的数据？
        let mut headers_str = "".to_string();
        for (k, v) in map.iter() {
            headers_str = format!("{}{}: {}\r\n", headers_str, k, v);
        }
        headers_str
    }

    fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_http_response() {
        let response = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Hello, World!".to_string()),
        };

        let response_expect_str: String = response.into();
        let response_str = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\nHello, World!";
        assert_eq!(response_expect_str, response_str)
    }

    // fn test_send_response() {
    //     let mut stream = Vec::new();
    //     let response = HttpResponse::new(
    //         "200",
    //         Some(HashMap::new()),
    //         Some("Hello, World!".to_string()),
    //     );
    //     response.send_response(&mut stream).unwrap();
    //     assert_eq!(stream, b"HTTP/1.1 200 OK\r\n\r\nHello, World!\n");
    // }
}
