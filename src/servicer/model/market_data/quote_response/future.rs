use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, TimestampMilliSeconds};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureResponse {
    pub asset_main_type: String,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    pub quote: QuoteFuture,
    pub reference: ReferenceFuture,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteFuture {
    #[serde(rename = "askMICId")]
    pub ask_micid: Option<String>,
    pub ask_price: f64,
    pub ask_size: i64,
	#[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub ask_time: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "bidMICId")]
    pub bid_micid: Option<String>,
    pub bid_price: f64,
    pub bid_size: i64,
	#[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub bid_time: chrono::DateTime<chrono::Utc>,
    pub close_price: i64,
    pub future_percent_change: f64,
    pub high_price: i64,
    #[serde(rename = "lastMICId")]
    pub last_micid: Option<String>,
    pub last_price: f64,
    pub last_size: i64,
    pub low_price: f64,
    pub mark: i64,
    pub net_change: f64,
    pub open_interest: i64,
    pub open_price: f64,
	#[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub quote_time: chrono::DateTime<chrono::Utc>,
    pub quote_in_session: Option<bool>,
    pub security_status: String,
	#[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub settle_time: chrono::DateTime<chrono::Utc>,
    pub tick: f64,
    pub tick_amount: f64,
    pub total_volume: i64,
	#[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceFuture {
    pub description: String,
    pub exchange: String,
    pub exchange_name: String,
    pub future_active_symbol: String,
	#[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub future_expiration_date: chrono::DateTime<chrono::Utc>,
    pub future_is_active: bool,
    pub future_multiplier: i64,
    pub future_price_format: String,
    pub future_settlement_price: i64,
    pub future_trading_hours: String,
    pub product: String,
    // not in schama
    pub future_is_tradable: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse/FutureResponse.json"
        ));

        let val = serde_json::from_str::<HashMap<String, FutureResponse>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
