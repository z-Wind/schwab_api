use serde::Deserialize;
use serde::Serialize;

use crate::Number;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mover {
    pub screeners: Vec<Screener>,
}

/// Security info of most moved with in an index
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Screener {
    /// percent or value changed, by default its percent changed
    pub change: Option<Number>,
    /// Name of security
    pub description: String,
    pub direction: Option<Direction>,
    /// what was last quoted price
    pub last: Option<Number>,
    /// schwab security symbol
    pub symbol: String,
    pub total_volume: u64,

    // Fields not explicitly defined in the official schema
    // ===================================================
    pub volume: Option<u64>,
    pub last_price: Option<Number>,
    pub net_change: Option<Number>,
    pub market_share: Option<Number>,
    pub trades: Option<i64>,
    pub net_percent_change: Option<Number>,
    // ===================================================
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Up,
    Down,
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
            "/tests/model/MarketData/Mover.json"
        ));

        let val = serde_json::from_str::<Mover>(json);
        tracing::debug!(?val);
        assert!(val.is_ok(), "Failed to deserialize: {:?}", val.err());
    }

    #[test]
    fn test_serde_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/Mover_real.json"
        ));
        let json: serde_json::Value = serde_json::from_str(json).unwrap();

        let val = serde_json::from_value::<Mover>(json.clone()).unwrap();
        tracing::debug!(?val);

        assert_json_matches!(
            val,
            json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat)
        );
    }
}
