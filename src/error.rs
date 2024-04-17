#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported operation: {0}")]
    UnSupported(String),
    #[error("Standard I/O error: {0}")]
    STDIO(#[from] std::io::Error),
    #[error("Token error: {0}")]
    TokenError(String),
    #[error("HTTP request failed with status code: {0} {0}")]
    HttpRequestFailed(String, reqwest::StatusCode),
    #[error("Empty response text for symbols: {0}")]
    EmptyResponseText(String),
    #[error("No match for JSON format in response text: {0}")]
    NoMatchJsonFormat(String),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
	#[error("JSON error: {0}")]
	Json(#[from] serde_json::Error),
	#[error("ErrorResponse: {0:?}")]
    ErrorResponse(crate::servicer::model::ErrorResponse),
	#[error("ServiceError: {0:?}")]
    ServiceError(crate::servicer::model::ServiceError),
}
