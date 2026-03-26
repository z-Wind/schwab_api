use serde::Deserialize;
use serde::Serialize;

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceError {
    pub message: Option<String>,
    pub errors: Option<Vec<ErrorDetail>>,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref msg) = self.message {
            write!(f, "Message: {msg} ")?;
        }
        if let Some(ref details) = self.errors {
            for err in details {
                write!(f, "[{}: {}] ", err.title, err.detail)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetail {
    pub id: String,
    pub status: i64,
    pub title: String,
    pub detail: String,
}

#[cfg(test)]
mod tests {
    use assert_json_diff::{CompareMode, Config, NumericMode, assert_json_matches};
    use test_log::test;

    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/ServiceError.json"
        ));

        let val = serde_json::from_str::<ServiceError>(json);
        tracing::debug!(?val);
        assert!(val.is_ok(), "Failed to deserialize: {:?}", val.err());
    }

    #[test]
    fn test_serde_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/ServiceError_real.json"
        ));
        let json: serde_json::Value = serde_json::from_str(json).unwrap();

        let val = serde_json::from_value::<ServiceError>(json.clone()).unwrap();
        tracing::debug!(?val);

        assert_json_matches!(
            val,
            json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat)
        );
    }

    #[test]
    fn test_serde_real2() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/ServiceError_real2.json"
        ));
        let json: serde_json::Value = serde_json::from_str(json).unwrap();

        let val = serde_json::from_value::<ServiceError>(json.clone()).unwrap();
        tracing::debug!(?val);

        assert_json_matches!(
            val,
            json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat)
        );
    }
}
