use actix_web::{web, Error as AWError};
use anyhow::Error;
use futures::{Future, TryFutureExt};
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;

pub type Pool = r2d2::Pool<PostgresConnectionManager<NoTls>>;

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

pub fn get_pool() -> Pool {
    let conn_str = get_connurl();
    let manager = PostgresConnectionManager::new(conn_str.parse().unwrap(), NoTls);
    Pool::new(manager).unwrap()
}

fn db_getter(
    db_pool: web::Data<Pool>,
    anon: bool,
    scheme: Option<String>,
) -> Result<String, Error> {
    let mut client = db_pool.get()?;
    let anon_cond = if anon { " AND anon = true" } else { "" };
    let scheme_cond = if let Some(scheme) = scheme {
        format!(" AND scheme = '{}'", scheme)
    } else {
        String::new()
    };
    let stmt = format!(
        "
        SELECT
            hostname
        FROM
            proxies
        TABLESAMPLE SYSTEM(1)
        WHERE
            work = true{}{}
        LIMIT 1;
    ",
        anon_cond, scheme_cond
    );
    let row = client.query_opt(stmt.as_str(), &[])?;
    let hostname = if let Some(row) = row {
        row.get("hostname")
    } else {
        String::new()
    };
    Ok(hostname)
}

pub fn get_proxy(
    db_pool: web::Data<Pool>,
    anon: bool,
    scheme: Option<String>,
) -> impl Future<Output = Result<String, AWError>> {
    web::block(move || db_getter(db_pool, anon, scheme)).map_err(AWError::from)
}
