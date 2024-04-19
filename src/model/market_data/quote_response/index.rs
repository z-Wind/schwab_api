use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, TimestampMilliSeconds};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexResponse {
    pub asset_main_type: String,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    pub quote: QuoteIndex,
    pub reference: ReferenceIndex,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteIndex {
    #[serde(rename = "52WeekHigh")]
    pub n52week_high: f64,
    #[serde(rename = "52WeekLow")]
    pub n52week_low: f64,
    pub close_price: f64,
    pub high_price: f64,
    pub last_price: f64,
    pub low_price: f64,
    pub net_change: f64,
    pub net_percent_change: f64,
    pub open_price: f64,
    pub security_status: String,
    pub total_volume: i64,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceIndex {
    pub description: String,
    pub exchange: String,
    pub exchange_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse/IndexResponse.json"
        ));

        let val = serde_json::from_str::<HashMap<String, IndexResponse>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
