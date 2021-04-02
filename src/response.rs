use bytes::Bytes;

use crate::{headers::Headers, status::StatusCode, version::Version};

#[derive(Debug, PartialEq, Clone)]
pub struct Response {
    pub version: Version,
    pub status_code: StatusCode,
    pub headers: Headers,
    pub body: Bytes,
}

impl Response {
    pub fn new(body: String) -> Self {
        let mut response = Response::default();
        response.body(Bytes::from(body));
        response
    }

    pub fn status_code(&mut self, code: u16) {
        self.status_code = StatusCode::from(code);
    }

    pub fn version(&mut self, version: Version) {
        self.version = version;
    }

    pub fn headers(&mut self, headers: Headers) {
        self.headers = headers;
    }

    pub fn body(&mut self, body: Bytes) {
        self.headers.insert("Content-Length", &body.len());
        self.body = body;
    }
}

impl Default for Response {
    fn default() -> Self {
        let version = Version::default();
        let status_code = StatusCode::from(200);
        let headers = Headers::new();
        let body = Bytes::new();

        Response {
            version,
            status_code,
            headers,
            body,
        }
    }
}
