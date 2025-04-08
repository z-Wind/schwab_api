use serde::Deserialize;
use serde::Serialize;
use serde_with::{TimestampMilliSeconds, serde_as};

#[serde_as]
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandleList {
    pub candles: Vec<Candle>,
    pub empty: Option<bool>,
    pub previous_close: Option<f64>,
    #[serde_as(as = "Option<TimestampMilliSeconds<i64>>")]
    pub previous_close_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "previousCloseDateISO8601")]
    pub previous_close_date_iso8601: Option<chrono::DateTime<chrono::Utc>>,
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    pub close: f64,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub datetime: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "dateTimeISO8601", skip_serializing_if = "Option::is_none")]
    pub datetime_iso8601: Option<chrono::DateTime<chrono::Utc>>,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub volume: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::{CompareMode, Config, NumericMode, assert_json_matches};

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/CandleList.json"
        ));

        let val = serde_json::from_str::<CandleList>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_serde_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/CandleList_real.json"
        ));
        let json: serde_json::Value = serde_json::from_str(json).unwrap();

        let val = serde_json::from_value::<CandleList>(json.clone()).unwrap();
        dbg!(&val);

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
            "/tests/model/MarketData/CandleList_real2.json"
        ));
        let json: serde_json::Value = serde_json::from_str(json).unwrap();

        let val = serde_json::from_value::<CandleList>(json.clone()).unwrap();
        // dbg!(&val);

        assert_json_matches!(
            val,
            json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat)
        );
    }
}
