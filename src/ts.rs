use tokio::{io::AsyncWriteExt, net::TcpListener};

use crate::{
    db::get_pool,
    error::ChError,
    request::Request,
    response::Response,
    routes::{check, proxy},
};

pub async fn tserver() -> Result<(), ChError> {
    dotenv::dotenv().ok();
    let pool = get_pool();
    let listener = TcpListener::bind("127.0.0.1:18080").await?;
    loop {
        let pool = pool.clone();
        let (mut socket, client) = listener.accept().await?;
        tokio::spawn(async move {
            let req = Request::from_stream(&mut socket).await?;
            let version = req.version();
            let response = match req.request_uri() {
                "/c" => check(&req, client).await?,
                "/pn" => proxy(pool.clone(), false, None, version).await?,
                "/pnh" => proxy(pool.clone(), false, Some("http"), version).await?,
                "/pns" => proxy(pool.clone(), false, Some("socks5"), version).await?,
                "/pa" => proxy(pool.clone(), true, None, version).await?,
                "/pah" => proxy(pool.clone(), true, Some("http"), version).await?,
                "/pas" => proxy(pool.clone(), true, Some("socks5"), version).await?,
                _ => Response::default(),
            };
            socket.write_all(&response.to_bytes()).await
        });
    }
}
