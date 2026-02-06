use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{TimestampMilliSeconds, serde_as};

/// Quote info of Equity security
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquityResponse {
    /// nullable: true
    /// Asset Sub Type (only there if applicable)
    pub asset_sub_type: Option<EquityAssetSubType>,

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

    /// nullable: true
    ///
    /// NBBO - realtime, NFL - Non-fee liable quote.
    pub quote_type: QuoteType,
    pub extended: Option<ExtendedMarket>,
    pub fundamental: Option<Fundamental>,
    pub quote: QuoteEquity,
    pub reference: ReferenceEquity,
    pub regular: Option<RegularMarket>,
}

/// Quote data for extended hours
#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedMarket {
    /// example: 124.85
    ///
    /// Extended market ask price
    pub ask_price: f64,

    /// example: 51771
    ///
    /// Extended market ask size
    pub ask_size: i64,

    /// example: 124.85
    ///
    /// Extended market bid price
    pub bid_price: f64,

    /// example: 51771
    ///
    /// Extended market bid size
    pub bid_size: i64,

    /// example: 124.85
    ///
    /// Extended market last price
    pub last_price: f64,

    /// example: 51771
    ///
    /// Regular market last size
    pub last_size: i64,

    /// example: 1.1246
    ///
    /// mark price
    pub mark: f64,

    /// example: 1621368000400
    ///
    /// Extended market quote time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub quote_time: chrono::DateTime<chrono::Utc>,

    /// example: 12345
    ///
    /// Total volume
    pub total_volume: u64,

    /// example: 1621368000400
    ///
    /// Extended market trade time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,
}

/// Fundamentals of a security
#[serde_with::apply(
Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fundamental {
    /// Average 10 day volume
    pub avg_10_days_volume: f64,

    /// Average 1 day volume
    pub avg_1_year_volume: f64,

    /// example: 2021-04-28T00:00:00Z
    ///
    /// pattern: yyyy-MM-dd'T'HH:mm:ssZ
    ///
    /// Declaration date in yyyy-mm-ddThh:mm:ssZ
    pub declaration_date: Option<chrono::DateTime<chrono::Utc>>,

    /// example: 0.88
    ///
    /// Dividend Amount
    pub div_amount: f64,

    /// example: 2021-05-07T00:00:00Z
    ///
    /// Dividend date in yyyy-mm-ddThh:mm:ssZ
    pub div_ex_date: Option<chrono::DateTime<chrono::Utc>>,

    /// nullable: true
    ///
    /// Dividend frequency 1 – once a year or annually 2 – 2x a year or semi-annualy 3 - 3x a year (ex. ARCO, EBRPF) 4 – 4x a year or quarterly 6 - 6x per yr or every other month 11 – 11x a year (ex. FBND, FCOR) 12 – 12x a year or monthly
    pub div_freq: DivFrequency,

    /// example: 0.22
    ///
    /// Dividend Pay Amount
    pub div_pay_amount: f64,

    /// example: 2021-05-13T00:00:00Z
    ///
    /// pattern: yyyy-MM-dd'T'HH:mm:ssZ
    ///
    /// Dividend pay date in yyyy-mm-ddThh:mm:ssZ
    pub div_pay_date: Option<chrono::DateTime<chrono::Utc>>,

    /// example: 0.7
    ///
    /// Dividend yield
    pub div_yield: f64,

    /// example: 4.45645
    ///
    /// Earnings per Share
    pub eps: f64,

    /// example: -1
    ///
    /// Fund Leverage Factor + > 0 <-
    pub fund_leverage_factor: f64,

    /// nullable: true
    ///
    /// `FundStrategy` "A" - Active "L" - Leveraged "P" - Passive "Q" - Quantitative "S" - Short
    pub fund_strategy: Option<FundStrategy>,

    /// example: 2021-02-12T00:00:00Z
    ///
    /// pattern: yyyy-MM-dd'T'HH:mm:ssZ
    ///
    /// Next Dividend date
    pub next_div_ex_date: Option<chrono::DateTime<chrono::Utc>>,

    /// example: 2021-02-12T00:00:00Z
    ///
    /// pattern: yyyy-MM-dd'T'HH:mm:ssZ
    ///
    /// Next Dividend pay date
    pub next_div_pay_date: Option<chrono::DateTime<chrono::Utc>>,

    /// example: 28.599
    ///
    /// P/E Ratio
    pub pe_ratio: f64,

    // not in schema
    pub last_earnings_date: Option<chrono::DateTime<chrono::Utc>>,
}

/// Quote data of Equity security
#[serde_as]
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteEquity {
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

    /// example: XNYS
    ///
    /// ask MIC code
    #[serde(rename = "askMICId")]
    pub ask_micid: Option<String>,

    /// example: 124.63
    ///
    /// Current Best Ask Price
    pub ask_price: f64,

    /// example: 700
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

    /// example: 124.6
    ///
    /// Current Best Bid Price
    pub bid_price: f64,

    /// example: 300
    ///
    /// Number of shares for bid
    pub bid_size: i64,

    /// example: 1621376892336
    ///
    /// Last bid time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub bid_time: chrono::DateTime<chrono::Utc>,

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
    pub last_micid: Option<String>,

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

    /// example: -0.01
    ///
    /// Mark Price change
    pub mark_change: Option<f64>,

    /// example: -0.0189
    ///
    /// Mark Price percent change
    pub mark_percent_change: Option<f64>,

    /// example: -0.04
    ///
    /// Current Last-Prev Close
    pub net_change: f64,

    /// example: -0.0756
    ///
    /// Net Percentage Change
    pub net_percent_change: Option<f64>,

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

    /// example: 20171188
    ///
    /// Aggregated shares traded throughout the day, including pre/post market hours.
    pub total_volume: u64,

    /// example: 1621376731304
    ///
    /// Last trade time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,

    /// example: 0.0094
    ///
    /// Option Risk/Volatility Measurement
    pub volatility: Option<f64>,

    // not in schema
    pub post_market_change: Option<f64>,
    pub post_market_percent_change: Option<f64>,
}

/// Reference data of Equity security
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceEquity {
    /// example: A23456789
    ///
    /// CUSIP of Instrument
    pub cusip: String,

    /// example: Apple Inc. - Common Stock
    ///
    /// Description of Instrument
    pub description: String,

    /// example: q
    ///
    /// Exchange Code
    pub exchange: String,

    /// Exchange Name
    pub exchange_name: String,

    /// FSI Desc
    pub fsi_desc: Option<String>,

    /// example: 100
    ///
    /// Hard to borrow quantity.
    pub htb_quantity: Option<i32>,

    /// example: 4.5
    ///
    /// Hard to borrow rate.
    pub htb_rate: Option<f64>,

    /// example: false
    ///
    /// is Hard to borrow security.
    pub is_hard_to_borrow: Option<bool>,

    /// example: false
    ///
    /// is shortable security.
    pub is_shortable: Option<bool>,

    /// OTC Market Tier
    pub otc_market_tier: Option<String>,
}

/// Market info of security
#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarket {
    /// example: 124.85
    ///
    /// Regular market last price
    #[serde(rename = "regularMarketLastPrice")]
    pub last_price: f64,

    /// example: 51771
    ///
    /// Regular market last size
    #[serde(rename = "regularMarketLastSize")]
    pub last_size: i64,

    /// example: -1.42
    ///
    /// Regular market net change
    #[serde(rename = "regularMarketNetChange")]
    pub net_change: f64,

    /// example: -1.1246
    ///
    /// Regular market percent change
    #[serde(rename = "regularMarketPercentChange")]
    pub percent_change: Option<f64>,

    /// example: 1621368000400
    ///
    /// Regular market trade time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    #[serde(rename = "regularMarketTradeTime")]
    pub trade_time: chrono::DateTime<chrono::Utc>,
}

/// Asset Sub Type (only there if applicable)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EquityAssetSubType {
    Coe,
    Prf,
    Adr,
    Gdr,
    Cef,
    Etf,
    Etn,
    Uit,
    War,
    Rgt,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuoteType {
    /// NBBO - realtime
    Nbbo,
    /// NFL - Non-fee liable quote
    Nfl,
}

/// Dividend frequency
#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DivFrequency {
    /// null
    Zero = 0,

    /// 1 – once a year or annually
    One = 1,

    /// 2 – 2x a year or semi-annualy
    Two = 2,

    /// 3 - 3x a year (ex. ARCO, EBRPF)
    Three = 3,

    /// 4 – 4x a year or quarterly
    Four = 4,

    /// 6 - 6x per yr or every other month
    Six = 6,

    /// 11 – 11x a year (ex. FBND, FCOR)
    Eleven = 11,

    /// 12 – 12x a year or monthly
    Twelve = 12,
}

/// Fund Strategy
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FundStrategy {
    ///  "A" - Active
    #[serde(rename = "A")]
    Active,

    /// "L" - Leveraged
    #[serde(rename = "L")]
    Leveraged,

    /// "P" - Passive
    #[serde(rename = "P")]
    Passive,

    /// "Q" - Quantitative
    #[serde(rename = "Q")]
    Quantitative,

    /// "S" - Short
    #[serde(rename = "S")]
    Short,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use test_log::test;

    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse/EquityResponse.json"
        ));

        let val = serde_json::from_str::<HashMap<String, EquityResponse>>(json);
        tracing::debug!(?val);
        assert!(val.is_ok());
    }
}
