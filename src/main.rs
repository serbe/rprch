use actix_web::{web, App, Error, HttpRequest, HttpServer, Responder};

use db::{get_pool, get_proxy, Pool};

mod db;

async fn check(req: HttpRequest) -> impl Responder {
    let headers = req.headers();
    match req.peer_addr() {
        Some(peer_addr) => [
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
            format!("<p>RemoteAddr: {}</p>\r\n", peer_addr),
            |mut acc, h| match headers.get(*h) {
                Some(key) => {
                    acc.push_str(&format!("<p>{}: {:?}</p>\r\n", h, key));
                    acc
                }
                None => acc,
            },
        ),
        None => "no parse peer addr".to_string(),
    }
}

async fn proxy(db: web::Data<Pool>) -> Result<String, Error> {
    let result = get_proxy(db, false, None).await?;
    Ok(result)
}

async fn anon_proxy(db: web::Data<Pool>) -> Result<String, Error> {
    let result = get_proxy(db, true, None).await?;
    Ok(result)
}

async fn proxy_with_scheme(req: HttpRequest, db: web::Data<Pool>) -> Result<String, Error> {
    let scheme = &req.match_info()["scheme"];
    let result = get_proxy(db, false, Some(scheme.to_string())).await?;
    Ok(result)
}

async fn anon_proxy_with_scheme(req: HttpRequest, db: web::Data<Pool>) -> Result<String, Error> {
    let scheme = &req.match_info()["scheme"];
    let result = get_proxy(db, true, Some(scheme.to_string())).await?;
    Ok(result)
}

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
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .service(web::resource(&check_path).route(web::get().to(check)))
            .service(web::resource(&proxy_path).route(web::get().to(proxy)))
            .service(web::resource(&anon_proxy_path).route(web::get().to(anon_proxy)))
            .service(web::resource(&proxy_path_with_scheme).route(web::get().to(proxy_with_scheme)))
            .service(
                web::resource(&anon_proxy_path_with_scheme)
                    .route(web::get().to(anon_proxy_with_scheme)),
            )
    })
    .bind(s_addr)?
    .run()
    .await
}
