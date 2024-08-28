use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

pub type Markets = HashMap<String, HashMap<String, Hours>>;

#[allow(clippy::struct_field_names)]
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hours {
    pub date: chrono::NaiveDate,
    pub market_type: MarketType,
    pub exchange: Option<String>,
    pub category: Option<String>,
    pub product: String,
    pub product_name: Option<String>,
    pub is_open: bool,
    pub session_hours: Option<HashMap<String, Vec<Interval>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketType {
    Bond,
    Equity,
    Etf,
    Extended,
    Forex,
    Future,
    FutureOption,
    Fundamental,
    Index,
    Indicator,
    MutualFund,
    Option,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::{assert_json_matches_no_panic, CompareMode, Config, NumericMode};

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/Markets.json"
        ));

        let val = serde_json::from_str::<Markets>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_serde_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/Markets_real.json"
        ));
        let json: serde_json::Value = serde_json::from_str(json).unwrap();

        let val = serde_json::from_value::<Markets>(json.clone()).unwrap();
        dbg!(&val);

        let message = assert_json_matches_no_panic(
            &val,
            &json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat),
        )
        .unwrap_err();

        let re = regex::Regex::new(r"(?:json atoms at path.*start.*are not equal.*\n.*\n.*\n.*\n.*)|(?:json atoms at path.*end.*are not equal.*\n.*\n.*\n.*\n.*)").unwrap();
        let message = re.replace_all(&message, "");
        let message = message.trim();
        println!("{message}");
        assert_eq!(message, "");
    }
}
