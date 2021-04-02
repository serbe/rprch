use std::{fmt, str::FromStr};

use crate::{error::ChError, version::Version};

#[derive(PartialEq, Debug, Clone)]
pub struct Status {
    pub version: Version,
    pub code: StatusCode,
    pub reason: String,
}

impl Status {
    pub fn version(&self) -> Version {
        self.version
    }

    pub fn status_code(&self) -> StatusCode {
        self.code
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl<T, U, V> From<(T, U, V)> for Status
where
    Version: From<T>,
    V: ToString,
    StatusCode: From<U>,
{
    fn from(status: (T, U, V)) -> Status {
        Status {
            version: Version::from(status.0),
            code: StatusCode::from(status.1),
            reason: status.2.to_string(),
        }
    }
}

impl FromStr for Status {
    type Err = ChError;

    fn from_str(status_line: &str) -> Result<Self, Self::Err> {
        let mut status_line = status_line.trim().splitn(3, ' ');

        let version = status_line.next().ok_or(ChError::EmptyVersion)?.parse()?;
        let code: StatusCode = status_line
            .next()
            .ok_or(ChError::EmptyStatusCode)?
            .parse()?;
        let reason = status_line
            .next()
            .unwrap_or_else(|| code.reason().unwrap_or("Unknown"))
            .to_string();

        Ok(Status {
            version,
            code,
            reason,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct StatusCode(u16);

impl StatusCode {
    pub fn from_u16(code: u16) -> Result<StatusCode, ChError> {
        if !(100..600).contains(&code) {
            return Err(ChError::InvalidStatusCode(code));
        }

        Ok(StatusCode(code))
    }

    pub fn is_info(self) -> bool {
        self.0 >= 100 && self.0 < 200
    }

    pub fn is_success(self) -> bool {
        self.0 >= 200 && self.0 < 300
    }

    pub fn is_redirect(self) -> bool {
        self.0 >= 300 && self.0 < 400
    }

    pub fn is_client_err(self) -> bool {
        self.0 >= 400 && self.0 < 500
    }

    pub fn is_server_err(self) -> bool {
        self.0 >= 500 && self.0 < 600
    }

    pub fn is<F: FnOnce(u16) -> bool>(self, f: F) -> bool {
        f(self.0)
    }

    pub fn reason(self) -> Option<&'static str> {
        match self.0 {
            100 => Some("Continue"),
            101 => Some("Switching Protocols"),
            102 => Some("Processing"),
            200 => Some("OK"),
            201 => Some("Created"),
            202 => Some("Accepted"),
            203 => Some("Non Authoritative Information"),
            204 => Some("No Content"),
            205 => Some("Reset Content"),
            206 => Some("Partial Content"),
            207 => Some("Multi-Status"),
            208 => Some("Already Reported"),
            226 => Some("IM Used"),
            300 => Some("Multiple Choices"),
            301 => Some("Moved Permanently"),
            302 => Some("Found"),
            303 => Some("See Other"),
            304 => Some("Not Modified"),
            305 => Some("Use Proxy"),
            307 => Some("Temporary Redirect"),
            308 => Some("Permanent Redirect"),
            400 => Some("Bad Request"),
            401 => Some("Unauthorized"),
            402 => Some("Payment Required"),
            403 => Some("Forbidden"),
            404 => Some("Not Found"),
            405 => Some("Method Not Allowed"),
            406 => Some("Not Acceptable"),
            407 => Some("Proxy Authentication Required"),
            408 => Some("Request Timeout"),
            409 => Some("Conflict"),
            410 => Some("Gone"),
            411 => Some("Length Required"),
            412 => Some("Precondition Failed"),
            413 => Some("Payload Too Large"),
            414 => Some("URI Too Long"),
            415 => Some("Unsupported Media Type"),
            416 => Some("Range Not Satisfiable"),
            417 => Some("Expectation Failed"),
            418 => Some("I'm a teapot"),
            421 => Some("Misdirected Request"),
            422 => Some("Unprocessable Entity"),
            423 => Some("Locked"),
            424 => Some("Failed Dependency"),
            426 => Some("Upgrade Required"),
            428 => Some("Precondition Required"),
            429 => Some("Too Many Requests"),
            431 => Some("Request Header Fields Too Large"),
            451 => Some("Unavailable For Legal Reasons"),
            500 => Some("Internal Server Error"),
            501 => Some("Not Implemented"),
            502 => Some("Bad Gateway"),
            503 => Some("Service Unavailable"),
            504 => Some("Gateway Timeout"),
            505 => Some("HTTP Version Not Supported"),
            506 => Some("Variant Also Negotiates"),
            507 => Some("Insufficient Storage"),
            508 => Some("Loop Detected"),
            510 => Some("Not Extended"),
            511 => Some("Network Authentication Required"),
            _ => None,
        }
    }
}

impl From<StatusCode> for u16 {
    fn from(code: StatusCode) -> Self {
        code.0
    }
}

impl From<u16> for StatusCode {
    fn from(code: u16) -> Self {
        StatusCode(code)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for StatusCode {
    type Err = ChError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        StatusCode::from_u16(s.parse()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::convert::TryFrom;

    const STATUS_LINE: &str = "HTTP/1.1 200 OK";
    const VERSION: &str = "HTTP/1.1";
    const CODE: u16 = 200;
    const REASON: &str = "OK";
    const CODE_S: StatusCode = StatusCode(200);

    #[test]
    fn status_code_new() {
        assert_eq!(
            StatusCode::from_u16(200).unwrap(),
            StatusCode::from_u16(200).unwrap()
        );
        assert_ne!(StatusCode::from_u16(400).unwrap(), StatusCode(404));
    }

    #[test]
    fn status_code_info() {
        for i in 100..200 {
            assert!(StatusCode(i).is_info())
        }

        for i in (0..1000).filter(|&i| i < 100 || i >= 200) {
            assert!(!StatusCode(i).is_info())
        }
    }

    #[test]
    fn status_code_success() {
        for i in 200..300 {
            assert!(StatusCode(i).is_success())
        }

        for i in (0..1000).filter(|&i| i < 200 || i >= 300) {
            assert!(!StatusCode(i).is_success())
        }
    }

    #[test]
    fn status_code_redirect() {
        for i in 300..400 {
            assert!(StatusCode(i).is_redirect())
        }

        for i in (0..1000).filter(|&i| i < 300 || i >= 400) {
            assert!(!StatusCode(i).is_redirect())
        }
    }

    #[test]
    fn status_code_client_err() {
        for i in 400..500 {
            assert!(StatusCode(i).is_client_err())
        }

        for i in (0..1000).filter(|&i| i < 400 || i >= 500) {
            assert!(!StatusCode(i).is_client_err())
        }
    }

    #[test]
    fn status_code_server_err() {
        for i in 500..600 {
            assert!(StatusCode(i).is_server_err())
        }

        for i in (0..1000).filter(|&i| i < 500 || i >= 600) {
            assert!(!StatusCode(i).is_server_err())
        }
    }

    #[test]
    fn status_code_is() {
        let check = |i| i % 3 == 0;

        let code_1 = StatusCode(200);
        let code_2 = StatusCode(300);

        assert!(!code_1.is(check));
        assert!(code_2.is(check));
    }

    #[test]
    fn status_code_reason() {
        assert_eq!(StatusCode(200).reason(), Some("OK"));
        assert_eq!(StatusCode(400).reason(), Some("Bad Request"));
    }

    #[test]
    fn status_code_from() {
        assert_eq!(StatusCode::from(200), StatusCode(200));
    }

    #[test]
    fn u16_from_status_code() {
        assert_eq!(u16::from(CODE_S), 200);
    }

    #[test]
    fn status_code_display() {
        let code = format!("Status Code: {}", StatusCode(200));
        const CODE_EXPECT: &str = "Status Code: 200";

        assert_eq!(code, CODE_EXPECT);
    }

    #[test]
    fn status_code_from_str() {
        assert_eq!("200".parse::<StatusCode>().unwrap(), StatusCode(200));
        assert_ne!("400".parse::<StatusCode>().unwrap(), StatusCode(404));
    }

    #[test]
    fn status_from() {
        let version = Version::try_from(VERSION).unwrap();
        let status = Status::from((version, CODE, REASON));

        assert_eq!(status.version(), version);
        assert_eq!(status.status_code(), CODE_S);
        assert_eq!(status.reason(), REASON);
    }

    #[test]
    fn status_from_str() {
        let version = Version::try_from(VERSION).unwrap();
        let status = STATUS_LINE.parse::<Status>().unwrap();

        assert_eq!(status.version(), version);
        assert_eq!(status.status_code(), CODE_S);
        assert_eq!(status.reason(), REASON);
    }
}
