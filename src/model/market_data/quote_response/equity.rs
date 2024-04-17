use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampMilliSeconds};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquityResponse {
    pub asset_main_type: String,
    pub asset_sub_type: Option<String>,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    pub quote_type: String,
    pub extended: Option<ExtendedMarket>,
    pub fundamental: Option<Fundamental>,
    pub quote: QuoteEquity,
    pub reference: ReferenceEquity,
    pub regular: RegularMarket,
}

#[serde_as]
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedMarket {
    pub ask_price: f64,
    pub ask_size: i64,
    pub bid_price: f64,
    pub bid_size: i64,
    pub last_price: f64,
    pub last_size: i64,
    pub mark: f64,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub quote_time: chrono::DateTime<chrono::Utc>,
    pub total_volume: i64,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fundamental {
    #[serde(rename = "avg10DaysVolume")]
    pub avg10days_volume: f64,
    #[serde(rename = "avg1YearVolume")]
    pub avg1year_volume: f64,
    pub declaration_date: Option<chrono::DateTime<chrono::Utc>>,
    pub div_amount: f64,
    pub div_ex_date: Option<chrono::DateTime<chrono::Utc>>,
    pub div_freq: i64,
    pub div_pay_amount: f64,
    pub div_pay_date: Option<chrono::DateTime<chrono::Utc>>,
    pub div_yield: f64,
    pub eps: f64,
    pub fund_leverage_factor: f64,
    pub fund_strategy: Option<String>,
    pub next_div_ex_date: Option<chrono::DateTime<chrono::Utc>>,
    pub next_div_pay_date: Option<chrono::DateTime<chrono::Utc>>,
    pub pe_ratio: f64,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteEquity {
    #[serde(rename = "52WeekHigh")]
    pub n52week_high: f64,
    #[serde(rename = "52WeekLow")]
    pub n52week_low: f64,
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
    pub close_price: f64,
    pub high_price: f64,
    #[serde(rename = "lastMICId")]
    pub last_micid: Option<String>,
    pub last_price: f64,
    pub last_size: i64,
    pub low_price: f64,
    pub mark: f64,
    pub mark_change: Option<f64>,
    pub mark_percent_change: Option<f64>,
    pub net_change: f64,
    pub net_percent_change: Option<f64>,
    pub open_price: f64,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub quote_time: chrono::DateTime<chrono::Utc>,
    pub security_status: String,
    pub total_volume: i64,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,
    pub volatility: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceEquity {
    pub cusip: String,
    pub description: String,
    pub exchange: String,
    pub exchange_name: String,
    pub fsi_desc: Option<String>,
    pub htb_quantity: Option<i32>,
    pub htb_rate: Option<f64>,
    pub is_hard_to_borrow: Option<bool>,
    pub is_shortable: Option<bool>,
    pub otc_market_tier: Option<String>,
}

#[serde_as]
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarket {
    pub regular_market_last_price: f64,
    pub regular_market_last_size: i64,
    pub regular_market_net_change: f64,
    pub regular_market_percent_change: Option<f64>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub regular_market_trade_time: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse/EquityResponse.json"
        ));

        let val = serde_json::from_str::<HashMap<String, EquityResponse>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
