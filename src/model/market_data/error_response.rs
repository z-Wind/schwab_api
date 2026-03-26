use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub errors: Vec<Error>,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.errors.is_empty() {
            return write!(f, "ErrorResponse contains no errors");
        }

        let all_errors: Vec<String> = self
            .errors
            .iter()
            .map(std::string::ToString::to_string)
            .collect();

        write!(
            f,
            "API Error (Total {}): [{}]",
            self.errors.len(),
            all_errors.join(", ")
        )
    }
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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ id: {}, status: {}, title: {}, detail: {:?}, source: {:?} }}",
            self.id,
            self.status,
            self.title,
            self.detail.as_deref().unwrap_or("N/A"),
            self.source
        )
    }
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusCode {
    /// 400 Bad Request
    /// [[RFC7231, Section 6.5.1](https://tools.ietf.org/html/rfc7231#section-6.5.1)]
    BadRequest,

    /// 401 Unauthorized
    /// [[RFC7235, Section 3.1](https://tools.ietf.org/html/rfc7235#section-3.1)]
    Unauthorized,

    /// 404 Not Found
    /// [[RFC7231, Section 6.5.4](https://tools.ietf.org/html/rfc7231#section-6.5.4)]
    NotFound,

    /// 500 Internal Server Error
    /// [[RFC7231, Section 6.6.1](https://tools.ietf.org/html/rfc7231#section-6.6.1)]
    InternalServerError,

    Unknown(i32),
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadRequest => write!(f, "400 Bad Request"),
            Self::Unauthorized => write!(f, "401 Unauthorized"),
            Self::NotFound => write!(f, "404 Not Found"),
            Self::InternalServerError => write!(f, "500 Internal Server Error"),
            Self::Unknown(code) => write!(f, "[UNKNOWN_STATUS] {code}"),
        }
    }
}

impl From<i32> for StatusCode {
    fn from(code: i32) -> Self {
        match code {
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            404 => Self::NotFound,
            500 => Self::InternalServerError,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TempStatus<'a> {
    Int(i32),
    Str(&'a str),
}

impl From<TempStatus<'_>> for StatusCode {
    fn from(temp: TempStatus) -> Self {
        match temp {
            TempStatus::Int(i) => Self::from(i),
            TempStatus::Str(s) => Self::from(s.parse::<i32>().unwrap_or(-1)),
        }
    }
}

impl<'de> Deserialize<'de> for StatusCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        TempStatus::deserialize(deserializer).map(Into::into)
    }
}

impl Serialize for StatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let val = match *self {
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
            Self::Unknown(code) => code,
        };
        serializer.serialize_i32(val)
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
        assert!(val.is_ok(), "Failed to deserialize: {:?}", val.err());
    }
}
