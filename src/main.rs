mod db;
mod error;
mod ext;
mod ts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ext::server().await;
    ts::tserver().await;
    Ok(())
}
