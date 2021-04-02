use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::{
    db::get_pool,
    error::ChError,
    request::Request,
    response::Response,
    routes::{anon_proxy, check, proxy},
};

pub async fn tserver() -> Result<(), ChError> {
    dotenv::dotenv().ok();
    let pool = get_pool();
    let check_path = &dotenv::var("check_path")
        .expect("No found variable check_path like /checkpath in environment");
    let proxy_path = &dotenv::var("proxy_path")
        .expect("No found variable proxy_path like /proxypath in environment");
    let anon_proxy_path = &format!("{}/anon", proxy_path);
    // let proxy_path_with_scheme = &format!("{}/:scheme", proxy_path);
    // let anon_proxy_path_with_scheme = &format!("{}/anon/:scheme", proxy_path);
    let listener = TcpListener::bind("127.0.0.1:18080").await?;
    loop {
        let (mut socket, client) = listener.accept().await?;
        println!("client {}", client.to_string());

        let req = Request::from_stream(&mut socket).await?;

        let response = match req.request_uri() {
            check_path => check(&req, client).await?,
            proxy_path => proxy(&req, pool.clone()).await?,
            anon_proxy_path => anon_proxy(&req, pool.clone()).await?,
            n => Response::default(),
        };

        println!("{:?}", req);

        // tokio::spawn(async move {
        //     let mut buf = [0; 1024];

        //     // In a loop, read data from the socket and write the data back.
        //     loop {
        //         let n = match socket.read(&mut buf).await {
        //             // socket closed
        //             Ok(n) if n == 0 => return,
        //             Ok(n) => n,
        //             Err(e) => {
        //                 eprintln!("failed to read from socket; err = {:?}", e);
        //                 return;
        //             }
        //         };

        //         // Write the data back
        //         if let Err(e) = socket.write_all(&buf[0..n]).await {
        //             eprintln!("failed to write to socket; err = {:?}", e);
        //             return;
        //         }
        //     }
        // });
    }

    Ok(())
}
