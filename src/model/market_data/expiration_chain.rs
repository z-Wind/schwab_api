use serde::Deserialize;
use serde::Serialize;

use super::quote_response::option::ExpirationType;
use super::quote_response::option::SettlementType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpirationChain {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub expiration_list: Vec<Expiration>,
}

/// expiration type
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expiration {
    pub days_to_expiration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    pub expiration_type: ExpirationType,
    pub standard: bool,
    pub settlement_type: Option<SettlementType>,
    pub option_roots: Option<String>,

    // not in schema
    pub expiration_date: chrono::NaiveDate,
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
            "/tests/model/MarketData/ExpirationChain.json"
        ));

        let val = serde_json::from_str::<ExpirationChain>(json);
        tracing::debug!(?val);
        assert!(val.is_ok());
    }

    #[test]
    fn test_serde_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/ExpirationChain_real.json"
        ));
        let json: serde_json::Value = serde_json::from_str(json).unwrap();

        let val = serde_json::from_value::<ExpirationChain>(json.clone()).unwrap();
        tracing::debug!(?val);

        assert_json_matches!(
            val,
            json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat)
        );
    }
}
