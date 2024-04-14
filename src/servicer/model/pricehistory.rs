use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampMilliSeconds};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct History {
    pub(crate) candles: Vec<Candle>,
    pub(crate) symbol: String,
    pub(crate) empty: bool,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Candle {
    pub(crate) open: f64,
    pub(crate) high: f64,
    pub(crate) low: f64,
    pub(crate) close: f64,
    pub(crate) volume: u64,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub(crate) datetime: chrono::NaiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_history() {
        let s = r#"{
            "candles": [
                {
                    "open": 39.4,
                    "high": 39.56,
                    "low": 38.98,
                    "close": 39.3,
                    "volume": 1613400,
                    "datetime": 1047621600000
                },
                {
                    "open": 195.5,
                    "high": 196.95,
                    "low": 193.57,
                    "close": 195.89,
                    "volume": 4473782,
                    "datetime": 1678770000000
                }
            ],
            "symbol": "VTI",
            "empty": false
        }"#;
        let val = serde_json::from_str::<History>(s);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
