use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, TimestampMilliSeconds};

/// Quote info of Option security
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionResponse {
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
    pub quote: QuoteOption,
    pub reference: ReferenceOption,
}

/// Quote data of Option security
#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteOption {
    /// example: 145.09
    ///
    /// Higest price traded in the past 12 months, or 52 weeks
    #[serde(rename = "52WeekHigh")]
    pub n52week_high: Option<f64>,

    /// example: 77.581
    ///
    /// Lowest price traded in the past 12 months, or 52 weeks
    #[serde(rename = "52WeekLow")]
    pub n52week_low: Option<f64>,

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

    /// example: -0.0407
    ///
    /// Delta Value
    pub delta: f64,

    /// example: 0.0001
    ///
    /// Gamma Value
    pub gamma: f64,

    /// example: 126.99
    ///
    /// Day's high trade price
    pub high_price: f64,

    /// example: 126.99
    ///
    /// Indicative Ask Price applicable only for Indicative Option Symbols
    pub ind_ask_price: f64,

    /// example: 126.99
    ///
    /// Indicative Bid Price applicable only for Indicative Option Symbols
    pub ind_bid_price: f64,

    /// example: 126.99
    ///
    /// Indicative Quote Time in milliseconds since Epoch applicable only for Indicative Option Symbols
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub ind_quote_time: chrono::DateTime<chrono::Utc>,

    /// example: -0.0067
    ///
    /// Implied Yield
    pub implied_yield: f64,

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
    pub mark_change: f64,

    /// example: -0.0189
    ///
    /// Mark Price percent change
    pub mark_percent_change: f64,

    /// example: -947.96
    ///
    /// Money Intrinsic Value
    pub money_intrinsic_value: f64,

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
    pub open_interest: f64,

    /// example: 52.8
    ///
    /// Price at market open
    pub open_price: f64,

    /// example: 1621376892336
    ///
    /// Last quote time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub quote_time: chrono::DateTime<chrono::Utc>,

    /// example: -0.3732
    ///
    /// Rho Value
    pub rho: f64,

    /// example: Normal
    ///
    /// Status of security
    pub security_status: String,

    /// example: 12.275
    ///
    /// Theoretical option Value
    pub theoretical_option_value: f64,

    /// example: -0.315
    ///
    /// Theta Value
    pub theta: f64,

    /// example: 12.22
    ///
    /// Time Value
    pub time_value: f64,

    /// example: 20171188
    ///
    /// Aggregated shares traded throughout the day, including pre/post market hours.
    pub total_volume: u64,

    /// example: 1621376731304
    ///
    /// Last trade time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,

    /// example: 3247.96
    ///
    /// Underlying Price
    pub underlying_price: f64,

    /// example: 1.4455
    ///
    /// Vega Value
    pub vega: f64,

    /// example: 0.0094
    ///
    /// Option Risk/Volatility Measurement
    pub volatility: f64,
}

/// Reference data of Option security
#[serde_as]
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceOption {
    /// Indicates call or put
    pub contract_type: ContractType,

    /// example: 0AMZN.TK12300000
    ///
    /// CUSIP of Instrument
    pub cusip: Option<String>,

    /// example: 94
    ///
    /// Days to Expiration
    pub days_to_expiration: i64,

    /// example: $6024.37 cash in lieu of shares, 212 shares of AZN
    ///
    /// Unit of trade
    pub deliverables: Option<String>,

    /// example: AMZN Aug 20 2021 2300 Put
    ///
    /// Description of Instrument
    pub description: String,

    /// default: o
    ///
    /// Exchange Code
    pub exchange: String,

    /// Exchange Name
    pub exchange_name: String,

    /// option contract exercise type America or European
    pub exercise_type: Option<ExerciseType>,

    /// example: 20
    ///
    /// maximum: 31
    ///
    /// minimum: 1
    ///
    /// Expiration Day
    pub expiration_day: u8,

    /// example: 8
    ///
    /// maximum: 12
    ///
    /// minimum: 1
    ///
    /// Expiration Month
    pub expiration_month: u8,

    /// M for End Of Month Expiration Calendar Cycle. (To match the last business day of the month), Q for Quarterly expirations (last business day of the quarter month MAR/JUN/SEP/DEC), W for Weekly expiration (also called Friday Short Term Expirations) and S for Expires 3rd Friday of the month (also known as regular options).
    pub expiration_type: Option<ExpirationType>,

    /// example: 2021
    ///
    /// Expiration Year
    pub expiration_year: i64,

    /// example: true
    ///
    /// Is this contract part of the Penny Pilot program
    pub is_penny_pilot: bool,

    /// example: 1629504000000
    ///
    /// milliseconds since epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub last_trading_day: chrono::DateTime<chrono::Utc>,

    /// example: 100
    ///
    /// Option multiplier
    pub multiplier: f64,

    /// option contract settlement type AM or PM
    pub settlement_type: SettlementType,

    /// example: 2300
    ///
    /// Strike Price
    pub strike_price: f64,

    /// example: AMZN Aug 20 2021 2300 Put
    ///
    /// A company, index or fund name
    pub underlying: String,

    // not in schema
    pub uv_expiration_type: Option<String>,
}

/// Indicates call or put
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    #[serde(rename = "P")]
    Put,
    #[serde(rename = "C")]
    Call,
}

/// option contract exercise type America or European
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExerciseType {
    #[serde(rename = "A")]
    America,
    #[serde(rename = "E")]
    European,
}

/// M for End Of Month Expiration Calendar Cycle. (To match the last business day of the month), Q for Quarterly expirations (last business day of the quarter month MAR/JUN/SEP/DEC), W for Weekly expiration (also called Friday Short Term Expirations) and S for Expires 3rd Friday of the month (also known as regular options).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExpirationType {
    /// M for End Of Month Expiration Calendar Cycle. (To match the last business day of the month)
    #[serde(rename = "M")]
    Month,

    /// Q for Quarterly expirations (last business day of the quarter month MAR/JUN/SEP/DEC)
    #[serde(rename = "Q")]
    Quarterly,

    /// W for Weekly expiration (also called Friday Short Term Expirations)
    #[serde(rename = "W")]
    Weekly,

    /// S for Expires 3rd Friday of the month (also known as regular options)
    #[serde(rename = "S")]
    ThirdFriday,
}

/// option contract settlement type AM or PM
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SettlementType {
    #[serde(rename = "A")]
    AM,
    #[serde(rename = "P")]
    PM,
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
