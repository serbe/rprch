use std::convert::Infallible;
use std::net::SocketAddr;

// use actix_web::{web, App, Error, HttpRequest, HttpServer, Responder};
use hyper::{Body, Request, Response, Server, Method};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};

use db::{get_pool, get_proxy, Pool};

mod db;

async fn check(client: SocketAddr, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let headers = req.headers();
    let body = [
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
        );
        Ok(Response::new(Body::from(body)))
}

async fn proxy(db: Pool, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(get_proxy(db, false, None).await.unwrap())))
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_pool = get_pool();
    let proxy_path = dotenv::var("proxy_path")
        .expect("No found variable proxy_path like /proxypath in environment");
    let s_addr = dotenv::var("s_addr")
        .expect("No found variable s_addr like 127.0.0.1:10000 in environment");
    let check_path = dotenv::var("check_path")
        .expect("No found variable check_path like /checkpath in environment");
    let anon_proxy_path = format!("{}/anon", proxy_path);
    let proxy_path_with_scheme = format!("{}/{{scheme}}", proxy_path);
    let anon_proxy_path_with_scheme = format!("{}/anon/{{scheme}}", proxy_path);

    let make_service = make_service_fn(move |conn: &AddrStream| {
        let client = conn.remote_addr();
        async move { Ok::<_, Infallible>(service_fn(move |req| {
            match (req.method(), req.uri().path()) {
                (&Method::GET, proxy_path) => proxy(db_pool.clone(), req).await,
                (&Method::GET, check_path) => check(client.clone(), req).await,
                _ => {
                    Ok(Response::new(Body::from(String::new())))
                    // *response.status_mut() = StatusCode::NOT_FOUND;
                },
            };
        })) }
    });

    let addr = s_addr.parse().expect("No parse s_addr");

    let server = Server::bind(&addr).serve(make_service);

    server.await.unwrap();

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
