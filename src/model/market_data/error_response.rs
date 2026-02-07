use serde::Serialize;
use serde::{Deserialize, Deserializer};
use serde_repr::Serialize_repr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub errors: Vec<Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorSource {
    /// readOnly: true
    ///
    /// example: List [ `/data/attributes/symbols`, `/data/attributes/cusips`, `/data/attributes/ssids` ]
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
#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr)]
#[repr(i32)]
pub enum StatusCode {
    /// 400 Bad Request
    /// [[RFC7231, Section 6.5.1](https://tools.ietf.org/html/rfc7231#section-6.5.1)]
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

impl<'de> Deserialize<'de> for StatusCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 1. 先解析成通用 JSON 數值 (serde_json::Value) 或自定義 Enum
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum TempStatus {
            Int(i32),
            Str(String),
        }

        let temp = TempStatus::deserialize(deserializer)?;

        // 2. 將字串或數字統一轉為 i32
        let code = match temp {
            TempStatus::Int(i) => i,
            TempStatus::Str(s) => s.parse::<i32>().map_err(serde::de::Error::custom)?,
        };

        // 3. 對應回 StatusCode
        match code {
            400 => Ok(StatusCode::BadRequest),
            401 => Ok(StatusCode::Unauthorized),
            404 => Ok(StatusCode::NotFound),
            500 => Ok(StatusCode::InternalServerError),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown status code: {code}",
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/ErrorResponse.json"
        ));

        let val = serde_json::from_str::<ErrorResponse>(json);
        tracing::debug!(?val);
        assert!(val.is_ok());
    }
}
