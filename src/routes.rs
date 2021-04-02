use std::net::SocketAddr;

use deadpool_postgres::Pool;

use crate::{
    db::get_proxy, error::ChError, request::Request, response::Response, version::Version,
};

pub async fn check(req: &Request, client: SocketAddr) -> Result<Response, ChError> {
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

    Ok(Response::new(req.version(), body))
}

pub async fn proxy(
    pool: Pool,
    anon: bool,
    scheme: Option<&str>,
    version: Version,
) -> Result<Response, ChError> {
    let body = get_proxy(pool, anon, scheme.map(|v| v.to_string())).await?;
    Ok(Response::new(version, body))
}
