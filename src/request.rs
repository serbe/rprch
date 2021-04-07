use std::convert::TryFrom;

// use bytes::Bytes;
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::{error::ChError, headers::Headers, version::Version};

#[derive(Clone, Debug)]
pub struct Request {
    request_uri: String,
    version: Version,
    headers: Headers,
}

impl Request {
    pub async fn from_stream(stream: &mut TcpStream) -> Result<Request, ChError> {
        let mut header = Vec::with_capacity(512);
        while !(header.len() > 4 && header[header.len() - 4..] == b"\r\n\r\n"[..]) {
            header.push(stream.read_u8().await.or(Err(ChError::HeaderIncomplete))?);
            if header.len() > 1024 {
                return Err(ChError::HeaderToBig);
            }
        }
        let request = Request::from_header(&header)?;

        Ok(request)
    }

    pub fn from_header(header: &[u8]) -> Result<Request, ChError> {
        let mut header = std::str::from_utf8(header)?.lines();
        let mut request_line = header.next().ok_or(ChError::EmptyHeader)?.trim().split(' ');
        let method = request_line.next().ok_or(ChError::EmptyMethod)?;
        if method != "GET" {
            return Err(ChError::MethodUnsupported(method.to_string()));
        }
        let request_uri = request_line
            .next()
            .ok_or(ChError::EmptyRequestUri)?
            .to_string();
        let version = Version::try_from(request_line.next())?;

        if request_line.next().is_some() {
            return Err(ChError::RequestLineToBig);
        }

        let headers = Headers::try_from(header)?;

        Ok(Request {
            request_uri,
            version,
            headers,
        })
    }

    pub fn headers(&self) -> Headers {
        self.headers.clone()
    }

    pub fn request_uri(&self) -> &str {
        &self.request_uri
    }

    pub fn version(&self) -> Version {
        self.version
    }
}
