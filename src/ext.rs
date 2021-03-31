use deadpool_postgres::Pool;
use hyper::{Body, Request, Response, Server};
use routerify::ext::RequestExt;
use routerify::{Router, RouterService};

use crate::db::{get_pool, get_proxy};
use crate::error::ChError;

struct DB {
    pub pool: Pool,
}

impl DB {
    fn new(pool: Pool) -> Self {
        DB { pool }
    }
}

async fn check(req: Request<Body>) -> Result<Response<Body>, ChError> {
    let client = req.remote_addr();
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

async fn proxy(req: Request<Body>) -> Result<Response<Body>, ChError> {
    let db = req.data::<DB>().ok_or(ChError::NoData)?;
    let body = get_proxy(db.pool.clone(), false, None).await?;
    Ok(Response::new(Body::from(body)))
}

async fn anon_proxy(req: Request<Body>) -> Result<Response<Body>, ChError> {
    let db = req.data::<DB>().ok_or(ChError::NoData)?;
    let body = get_proxy(db.pool.clone(), true, None).await?;
    Ok(Response::new(Body::from(body)))
}

async fn proxy_with_scheme(req: Request<Body>) -> Result<Response<Body>, ChError> {
    let db = req.data::<DB>().ok_or(ChError::NoData)?;
    let scheme = req.param("scheme").ok_or(ChError::NoParam)?;
    let body = get_proxy(db.pool.clone(), true, Some(scheme.to_string())).await?;
    Ok(Response::new(Body::from(body)))
}

async fn anon_proxy_with_scheme(req: Request<Body>) -> Result<Response<Body>, ChError> {
    let db = req.data::<DB>().ok_or(ChError::NoData)?;
    let scheme = req.param("scheme").ok_or(ChError::NoParam)?;
    let body = get_proxy(db.pool.clone(), true, Some(scheme.to_string())).await?;
    Ok(Response::new(Body::from(body)))
}

fn router() -> Router<Body, ChError> {
    let pool = get_pool();
    let proxy_path = dotenv::var("proxy_path")
        .expect("No found variable proxy_path like /proxypath in environment");
    let check_path = dotenv::var("check_path")
        .expect("No found variable check_path like /checkpath in environment");
    let anon_proxy_path = format!("{}/anon", proxy_path);
    let proxy_path_with_scheme = format!("{}/:scheme", proxy_path);
    let anon_proxy_path_with_scheme = format!("{}/anon/:scheme", proxy_path);
    Router::builder()
        .data(DB::new(pool))
        .get(proxy_path, proxy)
        .get(anon_proxy_path, anon_proxy)
        .get(proxy_path_with_scheme, proxy_with_scheme)
        .get(anon_proxy_path_with_scheme, anon_proxy_with_scheme)
        .get(check_path, check)
        .build()
        .unwrap()
}

pub async fn server() {
    dotenv::dotenv().ok();
    let s_addr = dotenv::var("s_addr")
        .expect("No found variable s_addr like 127.0.0.1:10000 in environment");
    let router = router();
    let addr = s_addr.parse().expect("No parse s_addr");
    let service = RouterService::new(router).unwrap();
    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
