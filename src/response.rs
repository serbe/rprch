use std::convert::TryInto;

use bytes::Bytes;

use crate::{headers::Headers, version::Version};

#[derive(Debug, PartialEq, Clone)]
pub struct Response {
    pub version: Version,
    pub status_code: u16,
    pub reason: String,
    pub headers: Headers,
    pub body: Bytes,
}

impl Response {
    pub fn new(version: Version, body: String) -> Self {
        let mut response = Response::default();
        response.version(version);
        response.body(Bytes::from(body));
        response
    }

    pub fn version(&mut self, version: Version) {
        self.version = version;
    }

    pub fn body<B>(&mut self, value: B)
    where
        B: TryInto<Bytes>,
    {
        match value.try_into() {
            Ok(body) => {
                self.headers.insert("Content-Length", &body.len());
                self.body = body;
            }
            _ => {
                self.body = Bytes::new();
                self.headers.remove("Content-Length");
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let status_line = format!(
            "{} {} {}{}",
            self.version, self.status_code, self.reason, "\r\n"
        );

        let headers: String = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}{}", k, v, "\r\n"))
            .collect();

        let mut response_msg = (status_line + &headers + "\r\n").as_bytes().to_vec();

        if !self.body.is_empty() {
            response_msg.extend(&self.body);
        }

        response_msg
    }
}

impl Default for Response {
    fn default() -> Self {
        let version = Version::default();
        let status_code = 200;
        let reason = "OK".to_string();
        let mut headers = Headers::new();
        headers.insert("Content-Type", "text/html");
        headers.insert("Connection", "close");
        let body = Bytes::new();

        Response {
            version,
            status_code,
            reason,
            headers,
            body,
        }
    }
}
