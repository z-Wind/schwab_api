use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, TimestampMilliSeconds};

/// Quote info of Index security
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexResponse {
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
    pub quote: QuoteIndex,
    pub reference: ReferenceIndex,
}

/// Quote data of Index security
#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteIndex {
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

    /// example: 52.74
    ///
    /// Day's low trade price
    pub low_price: f64,

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

    /// example: Normal
    ///
    /// Status of security
    pub security_status: String,

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

/// Reference data of Index security
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceIndex {
    /// example: DOW JONES 30 INDUSTRIALS
    ///
    /// Description of Instrument
    pub description: String,

    /// example: q
    ///
    /// Exchange Code
    pub exchange: String,

    /// Exchange Name
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
