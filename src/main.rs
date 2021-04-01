use std::convert::TryFrom;

use error::ChError;

// mod db;
mod error;
mod headers;
mod method;
mod request;
// mod ts;
mod version;

// mod ext;

#[tokio::main]
async fn main() -> Result<(), ChError> {
    // ext::server().await;
    // ts::tserver().await;
    let header = b"GET / HTTP/1.1 \r\nHost: alizar.habrahabr.ru\r\n\r\n";

    let request = request::Request::from_header(header);

    println!("{:?}", request);

    Ok(())
}
