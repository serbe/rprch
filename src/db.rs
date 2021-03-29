use anyhow::Error;
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};

fn get_config() -> Config {
    let mut config = Config::new();
    if let Ok(dbname) = dotenv::var("DB_NAME") {
        config.dbname(&dbname);
    };
    if let Ok(user) = dotenv::var("DB_USER") {
        config.user(&user);
    };
    if let Ok(password) = dotenv::var("DB_PASSWORD") {
        config.password(&password);
    };
    if let Ok(host) = dotenv::var("DB_HOST") {
        config.host(&host);
    };
    if let Ok(port) = dotenv::var("DB_PORT") {
        config.port(port.parse().expect("port need u16 type"));
    };
    config
}

pub fn get_pool() -> Pool {
    dotenv::dotenv().ok();
    let manager = Manager::new(get_config(), NoTls);
    Pool::new(manager, 16)
}

pub async fn get_proxy(db_pool: Pool, anon: bool, scheme: Option<String>) -> Result<String, Error> {
    let client = db_pool.get().await?;
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
    let row = client.query_opt(stmt.as_str(), &[]).await?;
    let hostname = if let Some(row) = row {
        row.get("hostname")
    } else {
        String::new()
    };
    Ok(hostname)
}
