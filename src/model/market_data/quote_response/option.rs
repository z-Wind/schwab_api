use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, TimestampMilliSeconds};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionResponse {
    pub asset_main_type: String,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    pub quote: QuoteOption,
    pub reference: ReferenceOption,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteOption {
    #[serde(rename = "52WeekHigh")]
    pub n52week_high: Option<f64>,
    #[serde(rename = "52WeekLow")]
    pub n52week_low: Option<f64>,
    pub ask_price: f64,
    pub ask_size: i64,
    pub bid_price: f64,
    pub bid_size: i64,
    pub close_price: f64,
    pub delta: f64,
    pub gamma: f64,
    pub high_price: f64,
    pub ind_ask_price: f64,
    pub ind_bid_price: f64,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub ind_quote_time: chrono::DateTime<chrono::Utc>,
    pub implied_yield: f64,
    pub last_price: f64,
    pub last_size: i64,
    pub low_price: f64,
    pub mark: f64,
    pub mark_change: f64,
    pub mark_percent_change: f64,
    pub money_intrinsic_value: f64,
    pub net_change: f64,
    pub net_percent_change: f64,
    pub open_interest: f64,
    pub open_price: f64,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub quote_time: chrono::DateTime<chrono::Utc>,
    pub rho: f64,
    pub security_status: String,
    pub theoretical_option_value: f64,
    pub theta: f64,
    pub time_value: f64,
    pub total_volume: i64,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,
    pub underlying_price: f64,
    pub vega: f64,
    pub volatility: f64,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceOption {
    pub contract_type: String,
    pub cusip: Option<String>,
    pub days_to_expiration: i64,
    pub deliverables: Option<String>,
    pub description: String,
    pub exchange: String,
    pub exchange_name: String,
    pub exercies_type: Option<String>,
    pub expiration_day: i64,
    pub expiration_month: i64,
    pub expiration_year: i64,
    pub is_penny_pilot: bool,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub last_trading_day: chrono::DateTime<chrono::Utc>,
    pub multiplier: i64,
    pub settlement_type: String,
    pub strike_price: f64,
    pub underlying: String,
    // not in schama
    pub uv_expiration_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse/OptionResponse.json"
        ));

        let val = serde_json::from_str::<HashMap<String, OptionResponse>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
