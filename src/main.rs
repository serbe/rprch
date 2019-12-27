use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

#[derive(Clone)]
struct DbPool {
    db: Pool<PostgresConnectionManager<NoTls>>,
}

struct Proxy {
    hostname: String,
}

fn get_connurl() -> String {
    let dbname = dotenv::var("DB_NAME").expect("missing env DB_NAME");
    let dbuser = dotenv::var("DB_USER");
    let dbpassword = dotenv::var("DB_PASSWORD");
    let dbhost = dotenv::var("DB_HOST");
    let dbport = dotenv::var("DB_PORT");
    let mut conn_str = format!("dbname={}", dbname);
    if let Ok(user) = dbuser {
        conn_str.push_str(format!(" user={}", user).as_str())
    };
    if let Ok(password) = dbpassword {
        conn_str.push_str(format!(" password={}", password).as_str())
    };
    if let Ok(host) = dbhost {
        conn_str.push_str(format!(" host={}", host).as_str())
    };
    if let Ok(port) = dbport {
        conn_str.push_str(format!(" port={}", port).as_str())
    };
    conn_str
}

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

async fn proxy(db_pool: web::Data<DbPool>) -> impl Responder {
    let mut pr = Proxy {
        hostname: String::new(),
    };
    if let Ok(mut client) = db_pool.db.get() {
        if let Ok(rows) = &client.query(
            "SELECT hostname FROM proxies TABLESAMPLE SYSTEM(1) WHERE work = true LIMIT 1",
            &[],
        ) {
            for row in rows {
                pr = Proxy {
                    hostname: row.get(0),
                }
            }
        }
    };
    pr.hostname
}

async fn anon_proxy(db_pool: web::Data<DbPool>) -> impl Responder {
    let mut pr = Proxy {
        hostname: String::new(),
    };
    if let Ok(mut client) = db_pool.db.get() {
        if let Ok(rows) = &client.query(
            "SELECT hostname FROM proxies TABLESAMPLE SYSTEM(1) WHERE anon = true AND work = true LIMIT 1",
            &[],
        ) {
            for row in rows {
                pr = Proxy {
                    hostname: row.get(0),
                }
            }
        }
    };
    pr.hostname
}

async fn proxy_with_scheme(req: HttpRequest, db_pool: web::Data<DbPool>) -> impl Responder {
    let mut pr = Proxy {
        hostname: String::new(),
    };
    let scheme = &req.match_info()["scheme"];
    if let Ok(mut client) = db_pool.db.get() {
        if let Ok(rows) = &client.query(
            "SELECT hostname FROM proxies TABLESAMPLE SYSTEM(1) WHERE work = true AND scheme = $1 LIMIT 1",
            &[&scheme],
        ) {
            for row in rows {
                pr = Proxy {
                    hostname: row.get(0),
                }
            }
        }
    };
    pr.hostname
}

async fn anon_proxy_with_scheme(req: HttpRequest, db_pool: web::Data<DbPool>) -> impl Responder {
    let mut pr = Proxy {
        hostname: String::new(),
    };
    let scheme = &req.match_info()["scheme"];
    if let Ok(mut client) = db_pool.db.get() {
        if let Ok(rows) = &client.query(
            "SELECT hostname FROM proxies TABLESAMPLE SYSTEM(1) WHERE anon = true AND work = true AND scheme = $1 LIMIT 1",
            &[&scheme],
        ) {
            for row in rows {
                pr = Proxy {
                    hostname: row.get(0),
                }
            }
        }
    };
    pr.hostname
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let conn_str = get_connurl();
    let manager = PostgresConnectionManager::new(conn_str.parse().unwrap(), NoTls);
    let pool = Pool::new(manager).unwrap();
    let db_pool = DbPool { db: pool.clone() };
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
