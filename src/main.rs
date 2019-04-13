use actix_web::{http, server, App, HttpRequest, Responder};

fn index(req: &HttpRequest) -> impl Responder {
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

fn main() {
    let addr =
        dotenv::var("s_addr").expect("No found variable s_addr like 127.0.0.1:10000 in environment");
    let path = dotenv::var("s_path").expect("No found variable s_path like /checkpath in environment");
    server::new(move || App::new().resource(&path, |r| r.method(http::Method::GET).f(index)))
        .bind(addr)
        .unwrap()
        .run();
}
