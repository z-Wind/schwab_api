use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, TimestampMilliSeconds};

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandleList {
    pub candles: Vec<Candle>,
    pub empty: bool,
    pub previous_close: f64,
	#[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub previous_close_date: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "previousCloseDateISO8601")]
    pub previous_close_date_iso8601: Option<chrono::DateTime<chrono::Utc>>,
    pub symbol: String,
}

#[serde_as]
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    pub close: f64,
	#[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub datetime: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "dateTimeISO8601")]
    pub datetime_iso8601: Option<chrono::DateTime<chrono::Utc>>,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub volume: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

	use std::collections::HashMap;

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
}