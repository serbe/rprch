use actix_web::{http, server, App, HttpRequest, Result};
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

#[derive(Clone)]
struct State {
    db: Pool<PostgresConnectionManager>,
}

struct Proxy {
    hostname: String,
}

fn check(req: &HttpRequest<State>) -> Result<String> {
    let headers = req.headers();
    match req.peer_addr() {
        Some(peer_addr) => {
            let res = [
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
                        acc.push_str(&format!("<p>{}: {:?}</p></p>\r\n", h, key));
                        acc
                    }
                    None => acc,
                },
            );
            Ok(res)
        }
        None => Ok("no parse peer addr".to_string())
    }
}

fn proxy(req: &HttpRequest<State>) -> Result<String> {
    let mut pr = Proxy {
        hostname: String::new(),
    };
    if let Ok(client) = req.state().db.get() {
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
    Ok(pr.hostname)
}

fn proxy_with_scheme(req: &HttpRequest<State>) -> Result<String> {
    let mut pr = Proxy {
        hostname: String::new(),
    };
    let scheme = &req.match_info()["scheme"];
    if let Ok(client) = req.state().db.get() {
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
    Ok(pr.hostname)
}

fn main() {
    let db_uri = dotenv::var("db_uri")
        .expect("No found variable db_uri like postgres://postgres@localhost:5433 in environment");
    let manager = PostgresConnectionManager::new(db_uri, TlsMode::None).unwrap();
    let pool = Pool::new(manager).unwrap();
    let state = State { db: pool.clone() };
    let proxy_path =
        dotenv::var("proxy_path").expect("No found variable proxy_path like /proxypath in environment");
    let s_addr = dotenv::var("s_addr")
        .expect("No found variable s_addr like 127.0.0.1:10000 in environment");
    let check_path = dotenv::var("check_path")
        .expect("No found variable check_path like /checkpath in environment");
    let proxy_path_with_scheme = format!("{}/{{scheme}}", proxy_path);
    server::new(move || {
        App::with_state(state.clone())
            .resource(&check_path, |r| r.method(http::Method::GET).f(check))
            .resource(&proxy_path, |r| r.method(http::Method::GET).f(proxy))
            .resource(&proxy_path_with_scheme, |r| r.method(http::Method::GET).f(proxy_with_scheme))
    })
    .bind(s_addr)
    .unwrap()
    .run();
}
