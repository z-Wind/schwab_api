use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, TimestampMilliSeconds};

use super::equity::Fundamental;

/// Quote info of Mutual Fund security
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualFundResponse {
    /// nullable: true
    /// Asset Sub Type (only there if applicable)
    pub asset_sub_type: Option<MutualFundAssetSubType>,

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
    pub fundamental: Option<Fundamental>,
    pub quote: QuoteMutualFund,
    pub reference: ReferenceMutualFund,
}

/// Quote data of Mutual Fund security
#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteMutualFund {
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
    /// Net Asset Value
    #[serde(rename = "nAV")]
    pub n_av: f64,

    /// example: -0.04
    ///
    /// Current Last-Prev Close
    pub net_change: f64,

    /// example: -0.0756
    ///
    /// Net Percentage Change
    pub net_percent_change: f64,

    /// example: Normal
    ///
    /// Status of security
    pub security_status: String,

    /// example: 20171188
    ///
    /// Aggregated shares taded throughout the day, including pre/post market hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<u64>,

    /// example: 1621376731304
    ///
    /// Last trade time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,

    // not in schema
    pub last_price: Option<f64>,
}

/// Reference data of Mutual Fund security
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceMutualFund {
    /// example: A23456789
    ///
    /// CUSIP of Instrument
    pub cusip: String,

    /// example: Apple Inc. - Common Stock
    ///
    /// Description of Instrument
    pub description: String,

    /// default: m
    ///
    /// Exchange Code
    pub exchange: String,

    /// default: `MUTUAL_FUND`
    ///
    /// Exchange Name
    pub exchange_name: String,
}

/// Asset Sub Type (only there if applicable)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MutualFundAssetSubType {
    Oef,
    Cef,
    Mmf,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse/MutualFundResponse.json"
        ));

        let val = serde_json::from_str::<HashMap<String, MutualFundResponse>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
