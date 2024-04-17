#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Standard I/O error: {0}")]
    STDIO(#[from] std::io::Error),
    #[error("Token error: {0}")]
    TokenError(String),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("ErrorResponse: {0:?}")]
    ErrorResponse(crate::model::ErrorResponse),
    #[error("ServiceError: {0:?}")]
    ServiceError(crate::model::ServiceError),
}
