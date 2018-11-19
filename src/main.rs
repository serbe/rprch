extern crate actix_web;

use actix_web::{http, server, App, HttpRequest, Responder};

fn index(req: &HttpRequest) -> impl Responder {
    let headers = req.headers();
    [
        "HTTP_VIA",
        "HTTP_X_FORWARDED_FOR",
        "HTTP_FORWARDED_FOR",
        "HTTP_X_FORWARDED",
        "HTTP_FORWARDED",
        "HTTP_CLIENT_IP",
        "HTTP_FORWARDED_FOR_IP",
        "VIA",
        "X_FORWARDED_FOR",
        "FORWARDED_FOR",
        "X_FORWARDED",
        "FORWARDED",
        "CLIENT_IP",
        "FORWARDED_FOR_IP",
        "HTTP_PROXY_CONNECTION",
    ]
    .iter()
    .fold(
        format!("<p>RemoteAddr: {}</p>\r\n", req.peer_addr().unwrap()),
        |mut acc, h| {
            if headers.contains_key(*h) {
                acc.push_str(&format!("<p>{}: {:?}</p></p>\r\n", h, headers.get(*h).unwrap()));
                acc
            } else {
                acc
            }
        },
    )
}

fn main() {
    server::new(|| App::new().resource("/check", |r| r.method(http::Method::GET).f(index)))
        .bind("127.0.0.1:16016")
        .unwrap()
        .run();
}
