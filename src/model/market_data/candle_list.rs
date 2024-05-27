use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, TimestampMilliSeconds};

#[serde_as]
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
    #[serde(rename = "dateTimeISO8601")]
    pub datetime_iso8601: Option<chrono::DateTime<chrono::Utc>>,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub volume: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_de_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/CandleList_real.json"
        ));

        let val = serde_json::from_str::<CandleList>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_de_real2() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/CandleList_real2.json"
        ));

        let val = serde_json::from_str::<CandleList>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
