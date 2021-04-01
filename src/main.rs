mod db;
mod error;
mod headers;
mod method;
mod request;
mod ts;
mod version;

// mod ext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ext::server().await;
    ts::tserver().await;
    Ok(())
}
