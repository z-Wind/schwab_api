use serde::Deserialize;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub errors: Vec<Error>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    /// readOnly: true
    ///
    /// example: 9821320c-8500-4edf-bd46-a9299c13d2e0
    ///
    /// Unique error id.
    pub id: String,

    /// readOnly: true
    ///
    /// example: 400
    ///
    /// The HTTP status code .
    pub status: StatusCode,

    /// readOnly: true
    ///
    /// example: Missing header
    ///
    /// Short error description.
    pub title: String,

    /// readOnly: true
    ///
    /// example: Search combination should not exceed 500.
    ///
    /// Detailed error description.
    pub detail: Option<String>,
    pub source: Option<ErrorSource>,
}

/// Who is responsible for triggering these errors.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorSource {
    /// readOnly: true
    ///
    /// example: List [ "/data/attributes/symbols", "/data/attributes/cusips", "/data/attributes/ssids" ]
    ///
    /// list of attributes which lead to this error message.
    pub pointer: Option<Vec<String>>,

    /// readOnly: true
    ///
    /// example: fields
    ///
    /// parameter name which lead to this error message.
    pub parameter: Option<String>,

    /// readOnly: true
    ///
    /// example: Schwab-Client-CorrelId
    ///
    /// header name which lead to this error message.
    pub header: Option<String>,
}

/// The HTTP status code .
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum StatusCode {
    /// 400 Bad Request
    /// [[RFC7231, Section 6.5.1](https://tools.ietf.org/html/rfc7231#section-6.5.1)]
    #[default]
    BadRequest = 400,

    /// 401 Unauthorized
    /// [[RFC7235, Section 3.1](https://tools.ietf.org/html/rfc7235#section-3.1)]
    Unauthorized = 401,

    /// 404 Not Found
    /// [[RFC7231, Section 6.5.4](https://tools.ietf.org/html/rfc7231#section-6.5.4)]
    NotFound = 404,

    /// 500 Internal Server Error
    /// [[RFC7231, Section 6.6.1](https://tools.ietf.org/html/rfc7231#section-6.6.1)]
    InternalServerError = 500,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/ErrorResponse.json"
        ));

        let val = serde_json::from_str::<ErrorResponse>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
