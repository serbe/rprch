use error::ChError;

mod db;
mod error;
mod headers;
mod method;
mod request;
mod response;
mod routes;
mod status;
mod ts;
mod version;

// mod ext;

#[tokio::main]
async fn main() -> Result<(), ChError> {
    // ext::server().await;
    ts::tserver().await
}
