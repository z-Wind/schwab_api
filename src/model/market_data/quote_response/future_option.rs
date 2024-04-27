use serde::Deserialize;
use serde::Serialize;

use super::option::ContractType;

/// Quote info of Future Option security
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureOptionResponse {
    /// example: 1234567890
    ///
    /// SSID of instrument
    pub ssid: i64,

    /// example: AAPL
    ///
    /// Symbol of instrument
    pub symbol: String,

    /// example: true
    ///
    /// is quote realtime
    pub realtime: bool,
    pub quote: QuoteFutureOption,
    pub reference: ReferenceFutureOption,
}

/// Quote data of Option security
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteFutureOption {
    /// example: XNYS
    ///
    /// ask MIC code
    #[serde(rename = "askMICId")]
    pub ask_micid: String,

    /// example: 124.63
    ///
    /// Current Best Ask Price
    pub ask_price: f64,

    /// example: 700
    ///
    /// Number of shares for ask
    pub ask_size: i64,

    /// example: XNYS
    ///
    /// bid MIC code
    #[serde(rename = "bidMICId")]
    pub bid_micid: String,

    /// example: 124.6
    ///
    /// Current Best Bid Price
    pub bid_price: f64,

    /// example: 300
    ///
    /// Number of shares for bid
    pub bid_size: i64,

    /// example: 126.27
    ///
    /// Previous day's closing price
    pub close_price: f64,

    /// example: 126.99
    ///
    /// Day's high trade price
    pub high_price: f64,

    /// example: XNYS
    ///
    /// Last MIC Code
    #[serde(rename = "lastMICId")]
    pub last_micid: String,

    /// example: 122.3
    pub last_price: f64,

    /// example: 100
    ///
    /// Number of shares traded with last trade
    pub last_size: i64,

    /// example: 52.74
    ///
    /// Day's low trade price
    pub low_price: f64,

    /// example: 52.93
    ///
    /// Mark price
    pub mark: i64,

    /// example: -0.04
    ///
    /// Mark Price change
    pub mark_change: f64,

    /// example: -0.04
    ///
    /// Current Last-Prev Close
    pub net_change: f64,

    /// example: -0.0756
    ///
    /// Net Percentage Change
    pub net_percent_change: f64,

    /// example: 317
    ///
    /// Open Interest
    pub open_interest: i64,

    /// example: 52.8
    ///
    /// Price at market open
    pub open_price: f64,

    /// example: 1621376892336
    ///
    /// Last quote time in milliseconds since Epoch
    pub quote_time: i64,

    /// example: Normal
    ///
    /// Status of security
    pub security_status: String,

    /// example: 52.8
    ///
    /// Price at market open
    pub settlemet_price: f64,

    /// example: 0
    ///
    /// Tick Price
    pub tick: f64,

    /// example: 0
    ///
    /// Tick Amount
    pub tick_amount: f64,

    /// example: 20171188
    ///
    /// Aggregated shares traded throughout the day, including pre/post market hours.
    pub total_volume: i64,

    /// example: 1621376731304
    ///
    /// Last trade time in milliseconds since Epoch
    pub trade_time: i64,
}

/// Reference data of Future Option security
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceFutureOption {
    /// Indicates call or put
    pub contract_type: ContractType,

    /// example: AMZN Aug 20 2021 2300 Put
    ///
    /// Description of Instrument
    pub description: String,

    /// example: q
    ///
    /// Exchange Code
    pub exchange: String,

    /// Exchange Name
    pub exchange_name: String,

    /// example: 100
    ///
    /// Option multiplier
    pub multiplier: f64,

    /// date of expiration in long
    pub expiration_date: i64,

    /// Style of expiration
    pub expiration_style: String,

    /// example: 2300
    ///
    /// Strike Price
    pub stricke_price: f64,

    /// example: AMZN Aug 20 2021 2300 Put
    ///
    /// A company, index or fund name
    pub underlying: String,
}
