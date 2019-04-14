use actix_web::{http, server, App, HttpRequest, Responder};
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

#[derive(Clone)]
struct State {
    db: Pool<PostgresConnectionManager>,
}

struct Proxy {
    hostname: String,
}

fn index1(req: &HttpRequest<State>) -> impl Responder {
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
        format!("<p>RemoteAddr: {}</p>\r\n", req.peer_addr().unwrap()),
        |mut acc, h| {
            if headers.contains_key(*h) {
                acc.push_str(&format!(
                    "<p>{}: {:?}</p></p>\r\n",
                    h,
                    headers.get(*h).unwrap()
                ));
                acc
            } else {
                acc
            }
        },
    )
}

fn index2(req: &HttpRequest<State>) -> impl Responder {
    let client = req.state().db.get().unwrap();
    let mut pr = Proxy {
        hostname: String::new(),
    };
    for row in &client
        .query(
            "SELECT hostname FROM proxies TABLESAMPLE SYSTEM(0.1) LIMIT 1",
            &[],
        )
        .unwrap()
    {
        pr = Proxy {
            hostname: row.get(0),
        }
    }

    // let c = Contact::get(&pool, 2).unwrap();
    // let data = JsonData {
    //     data: c,
    //     error: None,
    //     ok: true,
    // };
    // // println!("Request number: {}\n{:?}", c.id, c.name);
    // HttpResponse::Ok().json(data)
    pr.hostname
}

fn main() {
    let db_uri = dotenv::var("db_uri")
        .expect("No found variable db_uri like postgres://postgres@localhost:5433 in environment");
    let manager = PostgresConnectionManager::new(db_uri, TlsMode::None).unwrap();
    let pool = Pool::new(manager).unwrap();
    let state = State { db: pool.clone() };
    let i2_path =
        dotenv::var("i2_path").expect("No found variable i2_path like /checkpath in environment");
    let s_addr = dotenv::var("s_addr")
        .expect("No found variable s_addr like 127.0.0.1:10000 in environment");
    let i1_path =
        dotenv::var("i1_path").expect("No found variable i1_path like /checkpath in environment");
    server::new(move || {
        App::with_state(state.clone())
            .resource(&i1_path, |r| r.method(http::Method::GET).f(index1))
            .resource(&i2_path, |r| r.method(http::Method::GET).f(index2))
    })
    .bind(s_addr)
    .unwrap()
    .run();
}
