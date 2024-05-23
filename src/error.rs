//! Represents all possible errors the Client might encounter.

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Standard I/O error: {0}")]
    Stdio(#[from] std::io::Error),
    #[error("Token error: {0}")]
    Token(String),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("QuoteError: {0:?}")]
    Quote(crate::model::QuoteError),
    #[error("ErrorResponse: {0:?}")]
    Response(crate::model::ErrorResponse),
    #[error("ServiceError: {0:?}")]
    Service(crate::model::ServiceError),
}
