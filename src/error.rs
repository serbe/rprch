pub type Result<T> = std::result::Result<T, ChError>;

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
    #[error("Parse headers: {0}")]
    HeaderParse(String),
    #[error("Unsupported version {0}")]
    VersionUnsupported(String),
    #[error("Header incomplete")]
    HeaderIncomplete,
    #[error("Header more when 1024")]
    HeaderToBig,
    #[error("Input output: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse utf8")]
    FromUtf8(#[from] std::str::Utf8Error),
    #[error("No request-uri")]
    NoRequestUri,
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
