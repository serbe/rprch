#[derive(Debug, thiserror::Error)]
pub enum ChError {
    #[error("Executing DB query: {0}")]
    PgError(#[from] tokio_postgres::Error),
    #[error("Deadpool: {0}")]
    PoolError(#[from] deadpool_postgres::PoolError),
    #[error("Request no have data")]
    NoData,
    #[error("Request no have param")]
    NoParam,
}

#[derive(Debug, thiserror::Error)]
pub enum TsError {
    #[error("Input output: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Executing DB query: {0}")]
    PgError(#[from] tokio_postgres::Error),
    #[error("Deadpool: {0}")]
    PoolError(#[from] deadpool_postgres::PoolError),
    #[error("Request no have data")]
    NoData,
    #[error("Request no have param")]
    NoParam,
}
