use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureOptionResponse {
    pub asset_main_type: String,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    pub quote: QuoteFutureOption,
    pub reference: ReferenceFutureOption,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteFutureOption {
    #[serde(rename = "askMICId")]
    pub ask_micid: String,
    pub ask_price: f64,
    pub ask_size: i64,
    #[serde(rename = "bidMICId")]
    pub bid_micid: String,
    pub bid_price: f64,
    pub bid_size: i64,
    pub close_price: i64,
    pub high_price: i64,
    #[serde(rename = "lastMICId")]
    pub last_micid: String,
    pub last_price: f64,
    pub last_size: i64,
    pub low_price: f64,
    pub mark: i64,
    pub mark_change: f64,
    pub net_change: f64,
    pub net_percent_change: f64,
    pub open_interest: i64,
    pub open_price: f64,
    pub quote_time: i64,
    pub security_status: String,
    pub settlemet_price: f64,
    pub tick: f64,
    pub tick_amount: f64,
    pub total_volume: i64,
    pub trade_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceFutureOption {
    pub contract_type: String,
    pub description: String,
    pub exchange: String,
    pub exchange_name: String,
    pub multiplier: f64,
    pub expiration_date: i64,
    pub expiration_style: String,
    pub stricke_price: f64,
    pub underlying: f64,
}
