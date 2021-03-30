use deadpool_postgres::Pool;
use std::net::SocketAddr;
use std::sync::Arc;

// use hyper::http::HttpConnector;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Method, Request, Response, Server, StatusCode};

use crate::db::{get_pool, get_proxy};
use crate::error::ChError;

// static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";

async fn check(req: Request<Body>, client: &SocketAddr) -> Result<Response<Body>, ChError> {
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

async fn proxy(pool: Arc<Pool>) -> Result<Response<Body>, ChError> {
    let body = get_proxy(*pool, false, None).await?;
    Ok(Response::new(Body::from(body)))
}

// async fn anon_proxy(req: Request<Body>) -> Result<Response<Body>, ChError> {
//     let db = req.data::<DB>().ok_or(ChError::NoData)?;
//     let body = get_proxy(db.pool.clone(), true, None).await?;
//     Ok(Response::new(Body::from(body)))
// }

// async fn proxy_with_scheme(req: Request<Body>) -> Result<Response<Body>, ChError> {
//     let db = req.data::<DB>().ok_or(ChError::NoData)?;
//     let scheme = req.param("scheme").ok_or(ChError::NoParam)?;
//     let body = get_proxy(db.pool.clone(), true, Some(scheme.to_string())).await?;
//     Ok(Response::new(Body::from(body)))
// }

// async fn anon_proxy_with_scheme(req: Request<Body>) -> Result<Response<Body>, ChError> {
//     let db = req.data::<DB>().ok_or(ChError::NoData)?;
//     let scheme = req.param("scheme").ok_or(ChError::NoParam)?;
//     let body = get_proxy(db.pool.clone(), true, Some(scheme.to_string())).await?;
//     Ok(Response::new(Body::from(body)))
// }

async fn response_examples(
    req: Request<Body>,
    client: SocketAddr,
    pool: Arc<Pool>,
) -> Result<Response<Body>, ChError> {
    let check_path = dotenv::var("check_path")
        .expect("No found variable check_path like /checkpath in environment");
    let proxy_path = dotenv::var("proxy_path")
        .expect("No found variable proxy_path like /proxypath in environment");
    let anon_proxy_path = format!("{}/anon", proxy_path);
    let proxy_path_with_scheme = format!("{}/:scheme", proxy_path);
    let anon_proxy_path_with_scheme = format!("{}/anon/:scheme", proxy_path);
    match (req.method(), req.uri().path()) {
        (&Method::GET, check_path) => check(req, &client).await,
        // (&Method::POST, "/json_api") => api_post_response(req).await,
        (&Method::GET, "/json_api") => proxy(pool).await,
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}

async fn hy_s() {
    let addr = "127.0.0.1:1337".parse().unwrap();
    let pool = Arc::new(get_pool());

    let new_service = make_service_fn(move |socket: &AddrStream| {
        let client = socket.remote_addr();
        let pool = pool.clone();
        async move {
            Ok::<_, ChError>(service_fn(move |req| {
                // Clone again to ensure that client outlives this closure.
                response_examples(req, client.to_owned(), pool.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
