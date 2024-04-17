use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, TimestampMilliSeconds};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexResponse {
    pub asset_main_type: String,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    pub quote: QuoteForex,
    pub reference: ReferenceForex,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteForex {
    #[serde(rename = "52WeekHigh")]
    pub n52week_high: f64,
    #[serde(rename = "52WeekLow")]
    pub n52week_low: f64,
    pub ask_price: f64,
    pub ask_size: i64,
    pub bid_price: f64,
    pub bid_size: i64,
    pub close_price: f64,
    pub high_price: f64,
    pub last_price: f64,
    pub last_size: i64,
    pub low_price: f64,
    pub mark: f64,
    pub net_change: f64,
    pub net_percent_change: i64,
    pub open_price: f64,
	#[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub quote_time: chrono::DateTime<chrono::Utc>,
    pub security_status: String,
    pub tick: i64,
    pub tick_amount: i64,
    pub total_volume: i64,
	#[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceForex {
    pub description: String,
    pub exchange: String,
    pub exchange_name: String,
    pub is_tradable: bool,
    pub market_maker: String,
    pub product: String,
    pub trading_hours: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse/ForexResponse.json"
        ));

        let val = serde_json::from_str::<HashMap<String, ForexResponse>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
