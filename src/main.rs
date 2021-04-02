mod db;
mod error;
mod headers;
mod request;
mod response;
mod routes;
mod ts;
mod version;

#[tokio::main]
async fn main() -> Result<(), error::ChError> {
    ts::tserver().await
}
