#[derive(Debug, thiserror::Error)]
pub enum ChError {
    #[error("Executing DB query: {0}")]
    PgError(#[from] tokio_postgres::Error),
    #[error("Deadpool: {0}")]
    PoolError(#[from] deadpool_postgres::PoolError),
    #[error("Parse headers: {0}")]
    HeaderParse(String),
    #[error("Unsupported version {0}")]
    VersionUnsupported(String),
    #[error("Unsupported method {0}")]
    MethodUnsupported(String),
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
    #[error("Parse int")]
    ParseInt(#[from] std::num::ParseIntError),
}

impl From<ChError> for std::io::Error {
    fn from(err: ChError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}
