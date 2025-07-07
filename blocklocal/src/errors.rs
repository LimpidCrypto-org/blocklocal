pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Service error: {0}")]
    ServiceError(#[from] crate::services::ServiceError),
    #[error("Strategy error: {0}")]
    StrategyError(#[from] crate::strategies::StrategyError),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
