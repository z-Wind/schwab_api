use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, TimestampMilliSeconds};

/// Quote info of Future security
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureResponse {
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
    pub quote: QuoteFuture,
    pub reference: ReferenceFuture,
}

/// Quote data of Future security
#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteFuture {
    /// example: XNYS
    ///
    /// ask MIC code
    #[serde(rename = "askMICId")]
    pub ask_micid: Option<String>,

    /// example: 4083.25
    ///
    /// Current Best Ask Price
    pub ask_price: f64,

    /// example: 36
    ///
    /// Number of shares for ask
    pub ask_size: i64,

    /// example: 1621376892336
    ///
    /// Last ask time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub ask_time: chrono::DateTime<chrono::Utc>,

    /// example: XNYS
    ///
    /// bid MIC code
    #[serde(rename = "bidMICId")]
    pub bid_micid: Option<String>,

    /// example: 4083
    ///
    /// Current Best Bid Price
    pub bid_price: f64,

    /// example: 18
    ///
    /// Number of shares for bid
    pub bid_size: i64,

    /// example: 1621376892336
    ///
    /// Last bid time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub bid_time: chrono::DateTime<chrono::Utc>,

    /// example: 4123
    ///
    /// Previous day's closing price
    pub close_price: f64,

    /// example: -0.0756
    ///
    /// Net Percentage Change
    pub future_percent_change: f64,

    /// example: 4123
    ///
    /// Day's high trade price
    pub high_price: f64,
    #[serde(rename = "lastMICId")]

    /// example: XNYS
    ///
    /// Last MIC Code
    pub last_micid: Option<String>,

    /// example: 4083
    pub last_price: f64,

    /// example: 7
    ///
    /// Number of shares traded with last trade
    pub last_size: i64,

    /// example: 4075.5
    ///
    /// Day's low trade price
    pub low_price: f64,

    /// example: 5438.25
    ///
    /// Mark price
    pub mark: f64,

    /// example: -40
    ///
    /// Current Last-Prev Close
    pub net_change: f64,

    /// example: 2517139
    ///
    /// Open interest
    pub open_interest: i64,

    /// example: 4114
    ///
    /// Price at market open
    pub open_price: f64,

    /// example: 1621427004585
    ///
    /// Last quote time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub quote_time: chrono::DateTime<chrono::Utc>,

    /// example: false
    ///
    /// quoted during trading session
    pub quote_in_session: Option<bool>,

    /// example: Normal
    ///
    /// Status of security
    pub security_status: String,

    /// example: 1621376892336
    ///
    /// settlement time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub settle_time: chrono::DateTime<chrono::Utc>,

    /// example: 0.25
    ///
    /// Tick Price
    pub tick: f64,

    /// example: 12.5
    ///
    /// Tick Amount
    pub tick_amount: f64,

    /// example: 20171188
    ///
    /// Aggregated shares traded throughout the day, including pre/post market hours.
    pub total_volume: u64,

    /// example: 1621376731304
    ///
    /// Last trade time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,
}

/// Reference data of Future security
#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceFuture {
    /// example: E-mini S&P 500 Index Futures,Jun-2021,ETH
    ///
    /// Description of Instrument
    pub description: String,

    /// example: q
    ///
    /// Exchange Code
    pub exchange: String,

    /// Exchange Name
    pub exchange_name: String,

    /// example: /ESM21
    ///
    /// Active symbol
    pub future_active_symbol: Option<String>,

    /// example: 1623988800000
    ///
    /// Future expiration date in milliseconds since epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub future_expiration_date: chrono::DateTime<chrono::Utc>,

    /// example: true
    ///
    /// Future is active
    pub future_is_active: bool,

    /// example: 50.0
    ///
    /// Future multiplier
    pub future_multiplier: f64,

    /// example: D,D
    ///
    /// Price format
    pub future_price_format: String,

    /// example: 4123
    ///
    /// Future Settlement Price
    pub future_settlement_price: f64,

    /// example: GLBX(de=1640;0=-1700151515301600;1=r-17001515r15301600d-15551640;7=d-16401555)
    ///
    /// Trading Hours
    pub future_trading_hours: String,

    /// example: /ES
    ///
    /// Futures product symbol
    pub product: String,

    // not in schama
    pub future_is_tradable: Option<bool>,
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
