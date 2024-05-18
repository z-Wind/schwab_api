use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, TimestampMilliSeconds};

/// Quote info of Forex security
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexResponse {
    /// example: 1234567890
    ///
    /// SSID of instrument
    pub ssid: Option<i64>,

    /// example: AAPL
    ///
    /// Symbol of instrument
    pub symbol: String,

    /// example: true
    ///
    /// is quote realtime
    pub realtime: bool,
    pub quote: QuoteForex,
    pub reference: ReferenceForex,
}

/// Quote data of Forex security
#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteForex {
    /// example: 145.09
    ///
    /// Higest price traded in the past 12 months, or 52 weeks
    #[serde(rename = "52WeekHigh")]
    pub n52week_high: f64,

    /// example: 77.581
    ///
    /// Lowest price traded in the past 12 months, or 52 weeks
    #[serde(rename = "52WeekLow")]
    pub n52week_low: f64,

    /// example: 124.63
    ///
    /// Current Best Ask Price
    pub ask_price: f64,

    /// example: 700
    ///
    /// Number of shares for ask
    pub ask_size: i64,

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
    pub mark: f64,

    /// example: -0.04
    ///
    /// Current Last-Prev Close
    pub net_change: f64,

    /// example: -0.0756
    ///
    /// Net Percentage Change
    pub net_percent_change: f64,

    /// example: 52.8
    ///
    /// Price at market open
    pub open_price: f64,

    /// example: 1621376892336
    ///
    /// Last quote time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub quote_time: chrono::DateTime<chrono::Utc>,

    /// example: Normal
    ///
    /// Status of security
    pub security_status: String,

    /// example: 0.0
    ///
    /// Tick Price
    pub tick: f64,

    /// example: 0.0
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
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,
}

/// Reference data of Forex security
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceForex {
    /// example: Euro/USDollar Spot
    ///
    /// Description of Instrument
    pub description: String,

    /// example: q
    ///
    /// Exchange Code
    pub exchange: String,

    /// Exchange Name
    pub exchange_name: String,

    /// example: true
    ///
    /// is FOREX tradable
    pub is_tradable: bool,

    /// Market marker
    pub market_maker: Option<String>,

    /// example: null
    ///
    /// Product name
    pub product: Option<String>,

    /// Trading hours
    pub trading_hours: Option<String>,
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
