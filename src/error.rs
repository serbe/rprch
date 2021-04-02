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
    #[error("Method is empty")]
    EmptyMethod,
    #[error("Headers is empty")]
    EmptyHeader,
    #[error("Version is empty")]
    EmptyVersion,
    #[error("Request uri is empty")]
    EmptyRequestUri,
    #[error("Request line more when 3 chunks")]
    RequestLineToBig,
    #[error("Status code is empty")]
    EmptyStatusCode,
    #[error("Response is empty")]
    EmptyResponse,
    #[error("Invalid status code {0}")]
    InvalidStatusCode(u16),
    #[error("Parse int")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Status line is empty")]
    EmptyStatus,
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
