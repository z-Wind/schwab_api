use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mover {
    pub screeners: Vec<Screener>,
}

/// Security info of most moved with in an index
/// #[serde_with::apply(
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Screener {
    /// percent or value changed, by default its percent changed
    pub change: Option<f64>,
    /// Name of security
    pub description: String,
    pub direction: Option<Direction>,
    /// what was last quoted price
    pub last: Option<f64>,
    /// schwab security symbol
    pub symbol: String,
    pub total_volume: u64,

    // not in schema
    pub volume: Option<u64>,
    pub last_price: Option<f64>,
    pub net_change: Option<f64>,
    pub market_share: Option<f64>,
    pub trades: Option<i64>,
    pub net_percent_change: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::{CompareMode, Config, NumericMode, assert_json_matches};

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/Mover.json"
        ));

        let val = serde_json::from_str::<Mover>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_serde_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/Mover_real.json"
        ));
        let json: serde_json::Value = serde_json::from_str(json).unwrap();

        let val = serde_json::from_value::<Mover>(json.clone()).unwrap();
        dbg!(&val);

        assert_json_matches!(
            val,
            json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat)
        );
    }
}
