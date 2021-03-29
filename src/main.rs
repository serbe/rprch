use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

use deadpool_postgres::Pool;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn, Service};
use hyper::{http, Body, Method, Request, Response, Server, StatusCode};

use db::{get_pool, get_proxy};

mod db;

// struct Checker {
//     proxy_path: String,
//     pool: Pool,
// }

// impl Service<Request<Body>> for Checker {
//     type Response = Response<Body>;
//     type Error = http::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, req: Request<Body>) -> Self::Future {
//         let pr = if req.method() == &Method::GET && req.uri().path() == &self.proxy_path {
//             proxy(self.pool.clone())
//         } else {
//             async { Ok::<_, anyhow::Error>(String::new()) }
//         };
//         // create the body
//         let body = Body::from("hello, world!\n");
//         // Create the HTTP response
//         let resp = Response::builder()
//             .status(StatusCode::OK)
//             .body(body)
//             .expect("Unable to create `http::Response`");

//         // create a response in a future.
//         let fut = async { Ok(resp) };

//         // Return the response as an immediate future
//         Box::pin(fut)
//     }
// }

fn check(client: SocketAddr, req: Request<Body>) -> String {
    let headers = req.headers();
    [
        "CLIENT_IP",
        "FORWARDED",
        "FORWARDED_FOR",
        "FORWARDED_FOR_IP",
        "HTTP_CLIENT_IP",
        "HTTP_FORWARDED",
        "HTTP_FORWARDED_FOR",
        "HTTP_FORWARDED_FOR_IP",
        "HTTP_PROXY_CONNECTION",
        "HTTP_VIA",
        "HTTP_X_FORWARDED",
        "HTTP_X_FORWARDED_FOR",
        "VIA",
        "X_FORWARDED",
        "X_FORWARDED_FOR",
    ]
    .iter()
    .fold(
        format!("<p>RemoteAddr: {}</p>\r\n", client.to_string()),
        |mut acc, h| match headers.get(*h) {
            Some(key) => {
                acc.push_str(&format!("<p>{}: {:?}</p>\r\n", h, key));
                acc
            }
            None => acc,
        },
    )
}

fn proxy(db: Pool) -> impl Future<Output = Result<String, anyhow::Error>> {
    get_proxy(db, false, None)
}

// async fn anon_proxy(db: Pool) -> Result<String, Error> {
//     let result = get_proxy(db, true, None).await?;
//     Ok(result)
// }

// async fn proxy_with_scheme(req: HttpRequest, db: Pool) -> Result<String, Error> {
//     let scheme = &req.match_info()["scheme"];
//     let result = get_proxy(db, false, Some(scheme.to_string())).await?;
//     Ok(result)
// }

// async fn anon_proxy_with_scheme(req: HttpRequest, db: Pool) -> Result<String, Error> {
//     let scheme = &req.match_info()["scheme"];
//     let result = get_proxy(db, true, Some(scheme.to_string())).await?;
//     Ok(result)
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let db_pool = get_pool();
    let proxy_path = dotenv::var("proxy_path")
        .expect("No found variable proxy_path like /proxypath in environment");
    let s_addr = dotenv::var("s_addr")
        .expect("No found variable s_addr like 127.0.0.1:10000 in environment");
    // let check_path = dotenv::var("check_path")
    //     .expect("No found variable check_path like /checkpath in environment");
    // let anon_proxy_path = format!("{}/anon", proxy_path);
    // let proxy_path_with_scheme = format!("{}/{{scheme}}", proxy_path);
    // let anon_proxy_path_with_scheme = format!("{}/anon/{{scheme}}", proxy_path);

    // https://github.com/djc/bb8/blob/main/postgres/examples/hyper.rs

    let make_service = make_service_fn(move |conn: &AddrStream| {
        let client = conn.remote_addr();
        let pool = db_pool.clone();
        async move {
            //         let db_pool3 = db_pool2.clone();

            Ok::<_, Infallible>(service_fn(move |req| async {
                //             let mut response = Response::new(Body::empty());
                //             // match (req.method(), req.uri().path()) {
                //             //     (&Method::GET, proxy_path) => match proxy(db_pool3.clone(), req).await {
                //             //         Ok(value) => *response.body_mut() = Body::from(value),
                //             //         Err(_) => *response.status_mut() = StatusCode::UNSUPPORTED_MEDIA_TYPE,
                //             //     },
                //             //     (&Method::GET, check_path) => {
                //             //         *response.body_mut() = Body::from(check(client.clone(), req));
                //             //     }
                //             //     _ => {
                //             //         *response.status_mut() = StatusCode::NOT_FOUND;
                //             //     }
                //             // };

                //             let body = proxy(db_pool3.clone(), req);
                //             //  {
                //             //     Ok(value) => *response.body_mut() = Body::from(value),
                //             //     Err(_) => *response.status_mut() = StatusCode::UNSUPPORTED_MEDIA_TYPE,
                //             // };

                //             Ok::<_, Infallible>(response)
            }))
        }
    });

    // let _addr = s_addr.parse().expect("No parse s_addr");

    // let server = Server::bind(&addr).serve(make_service);

    // server.await;

    Ok(())

    // HttpServer::new(move || {
    //     App::new()
    //         .data(db_pool.clone())
    //         // .service(web::resource(&check_path).route(web::get().to(check)))
    //         // .service(web::resource(&proxy_path).route(web::get().to(proxy)))
    //         .service(web::resource(&anon_proxy_path).route(web::get().to(anon_proxy)))
    //         .service(web::resource(&proxy_path_with_scheme).route(web::get().to(proxy_with_scheme)))
    //         .service(
    //             web::resource(&anon_proxy_path_with_scheme)
    //                 .route(web::get().to(anon_proxy_with_scheme)),
    //         )
    // })
    // .bind(s_addr)?
    // .run()
    // .await
}
