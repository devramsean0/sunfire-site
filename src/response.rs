pub struct Response {
    status: i16,
    headers: Vec<&'static str>,
    body: String,
}

impl Response {
    pub fn new(status: i16, headers: Vec<&'static str>, body: String) -> Self {
        Response { status, headers, body }
    }

    pub fn to_string(&self) -> String {
        let mut response = format!("HTTP/1.1 {}\r\n", self.status);
        for header in &self.headers {
            response.push_str(&format!("{}\r\n", header));
        }
        response.push_str("Server: SunfireSite.rs\r\n");
        response.push_str(format!("Content-Length: {}\r\n", self.body.len()).as_str());
        response.push_str("\r\n");
        response.push_str(self.body.as_str());
        response
    }
}