pub mod equity;
pub mod forex;
pub mod future;
pub mod future_option;
pub mod index;
pub mod mutual_fund;
pub mod option;
pub mod quote_error;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct QuoteResponseMap {
    #[serde(flatten)]
    pub(crate) responses: HashMap<String, QuoteResponse>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) errors: Option<quote_error::QuoteError>,
}

/// a (symbol, `QuoteResponse`) map. `SCHWis` an example key
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "assetMainType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuoteResponse {
    Bond(String),
    Equity(Box<equity::EquityResponse>),
    Forex(forex::ForexResponse),
    Future(future::FutureResponse),
    FutureOption(future_option::FutureOptionResponse),
    Index(index::IndexResponse),
    MutualFund(mutual_fund::MutualFundResponse),
    Option(Box<option::OptionResponse>),
}

impl QuoteResponse {
    /// Returns the symbol of the quote
    #[must_use]
    pub fn symbol(&self) -> &str {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => &x.symbol,
            QuoteResponse::Forex(x) => &x.symbol,
            QuoteResponse::Future(x) => &x.symbol,
            QuoteResponse::FutureOption(x) => &x.symbol,
            QuoteResponse::Index(x) => &x.symbol,
            QuoteResponse::MutualFund(x) => &x.symbol,
            QuoteResponse::Option(x) => &x.symbol,
        }
    }

    /// Returns the 52-week high price
    #[must_use]
    pub fn n52week_high(&self) -> Option<f64> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.n52week_high),
            QuoteResponse::Forex(x) => Some(x.quote.n52week_high),
            QuoteResponse::Index(x) => Some(x.quote.n52week_high),
            QuoteResponse::MutualFund(x) => Some(x.quote.n52week_high),
            QuoteResponse::Future(_)
            | QuoteResponse::FutureOption(_)
            | QuoteResponse::Option(_) => None,
        }
    }

    /// Returns the 52-week low price
    #[must_use]
    pub fn n52week_low(&self) -> Option<f64> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.n52week_low),
            QuoteResponse::Forex(x) => Some(x.quote.n52week_low),
            QuoteResponse::Index(x) => Some(x.quote.n52week_low),
            QuoteResponse::MutualFund(x) => Some(x.quote.n52week_low),
            QuoteResponse::Future(_)
            | QuoteResponse::FutureOption(_)
            | QuoteResponse::Option(_) => None,
        }
    }

    /// Returns the current best ask price
    #[must_use]
    pub fn ask_price(&self) -> Option<f64> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.ask_price),
            QuoteResponse::Forex(x) => Some(x.quote.ask_price),
            QuoteResponse::Future(x) => Some(x.quote.ask_price),
            QuoteResponse::FutureOption(x) => Some(x.quote.ask_price),
            QuoteResponse::Option(x) => Some(x.quote.ask_price),
            QuoteResponse::Index(_) | QuoteResponse::MutualFund(_) => None,
        }
    }

    /// Returns the number of shares for ask
    #[must_use]
    pub fn ask_size(&self) -> Option<i64> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.ask_size),
            QuoteResponse::Forex(x) => Some(x.quote.ask_size),
            QuoteResponse::Future(x) => Some(x.quote.ask_size),
            QuoteResponse::FutureOption(x) => Some(x.quote.ask_size),
            QuoteResponse::Option(x) => Some(x.quote.ask_size),
            QuoteResponse::Index(_) | QuoteResponse::MutualFund(_) => None,
        }
    }

    /// Returns the last ask time in Utc format
    #[must_use]
    pub fn ask_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.ask_time),
            QuoteResponse::Future(x) => Some(x.quote.ask_time),
            QuoteResponse::Forex(_)
            | QuoteResponse::FutureOption(_)
            | QuoteResponse::Index(_)
            | QuoteResponse::MutualFund(_)
            | QuoteResponse::Option(_) => None,
        }
    }

    /// Returns the current best bid price
    #[must_use]
    pub fn bid_price(&self) -> Option<f64> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.bid_price),
            QuoteResponse::Forex(x) => Some(x.quote.bid_price),
            QuoteResponse::Future(x) => Some(x.quote.bid_price),
            QuoteResponse::FutureOption(x) => Some(x.quote.bid_price),
            QuoteResponse::Option(x) => Some(x.quote.bid_price),
            QuoteResponse::Index(_) | QuoteResponse::MutualFund(_) => None,
        }
    }

    /// Returns the number of shares for bid
    #[must_use]
    pub fn bid_size(&self) -> Option<i64> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.bid_size),
            QuoteResponse::Forex(x) => Some(x.quote.bid_size),
            QuoteResponse::Future(x) => Some(x.quote.bid_size),
            QuoteResponse::FutureOption(x) => Some(x.quote.bid_size),
            QuoteResponse::Option(x) => Some(x.quote.bid_size),
            QuoteResponse::Index(_) | QuoteResponse::MutualFund(_) => None,
        }
    }

    /// Returns the last bid time in Utc format
    #[must_use]
    pub fn bid_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.bid_time),
            QuoteResponse::Future(x) => Some(x.quote.bid_time),
            QuoteResponse::Forex(_)
            | QuoteResponse::FutureOption(_)
            | QuoteResponse::Index(_)
            | QuoteResponse::MutualFund(_)
            | QuoteResponse::Option(_) => None,
        }
    }

    /// Returns the previous day's closing price
    #[must_use]
    pub fn close_price(&self) -> f64 {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => x.quote.close_price,
            QuoteResponse::Forex(x) => x.quote.close_price,
            QuoteResponse::Future(x) => x.quote.close_price,
            QuoteResponse::FutureOption(x) => x.quote.close_price,
            QuoteResponse::Index(x) => x.quote.close_price,
            QuoteResponse::MutualFund(x) => x.quote.close_price,
            QuoteResponse::Option(x) => x.quote.close_price,
        }
    }

    /// Returns the day's high trade price
    #[must_use]
    pub fn high_price(&self) -> Option<f64> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.high_price),
            QuoteResponse::Forex(x) => Some(x.quote.high_price),
            QuoteResponse::Future(x) => Some(x.quote.high_price),
            QuoteResponse::FutureOption(x) => Some(x.quote.high_price),
            QuoteResponse::Index(x) => Some(x.quote.high_price),
            QuoteResponse::MutualFund(_) => None,
            QuoteResponse::Option(x) => Some(x.quote.high_price),
        }
    }

    /// Returns the latest traded price
    #[must_use]
    pub fn last_price(&self) -> Option<f64> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.last_price),
            QuoteResponse::Forex(x) => Some(x.quote.last_price),
            QuoteResponse::Future(x) => Some(x.quote.last_price),
            QuoteResponse::FutureOption(x) => Some(x.quote.last_price),
            QuoteResponse::Index(x) => Some(x.quote.last_price),
            QuoteResponse::MutualFund(_) => None,
            QuoteResponse::Option(x) => Some(x.quote.last_price),
        }
    }

    /// Returns the number of shares traded with the last trade
    #[must_use]
    pub fn last_size(&self) -> Option<i64> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.last_size),
            QuoteResponse::Forex(x) => Some(x.quote.last_size),
            QuoteResponse::Future(x) => Some(x.quote.last_size),
            QuoteResponse::FutureOption(x) => Some(x.quote.last_size),
            QuoteResponse::Option(x) => Some(x.quote.last_size),
            QuoteResponse::Index(_) | QuoteResponse::MutualFund(_) => None,
        }
    }

    /// Returns the day's low trade price
    #[must_use]
    pub fn low_price(&self) -> Option<f64> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.low_price),
            QuoteResponse::Forex(x) => Some(x.quote.low_price),
            QuoteResponse::Future(x) => Some(x.quote.low_price),
            QuoteResponse::FutureOption(x) => Some(x.quote.low_price),
            QuoteResponse::Index(x) => Some(x.quote.low_price),
            QuoteResponse::MutualFund(_) => None,
            QuoteResponse::Option(x) => Some(x.quote.low_price),
        }
    }

    /// Returns the current last-prev close price difference
    #[must_use]
    pub fn net_change(&self) -> f64 {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => x.quote.net_change,
            QuoteResponse::Forex(x) => x.quote.net_change,
            QuoteResponse::Future(x) => x.quote.net_change,
            QuoteResponse::FutureOption(x) => x.quote.net_change,
            QuoteResponse::Index(x) => x.quote.net_change,
            QuoteResponse::MutualFund(x) => x.quote.net_change,
            QuoteResponse::Option(x) => x.quote.net_change,
        }
    }

    /// Returns the day's open trade price
    #[must_use]
    pub fn open_price(&self) -> Option<f64> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.open_price),
            QuoteResponse::Forex(x) => Some(x.quote.open_price),
            QuoteResponse::Future(x) => Some(x.quote.open_price),
            QuoteResponse::FutureOption(x) => Some(x.quote.open_price),
            QuoteResponse::Index(x) => Some(x.quote.open_price),
            QuoteResponse::MutualFund(_) => None,
            QuoteResponse::Option(x) => Some(x.quote.open_price),
        }
    }

    /// Returns the time of the latest quote in Utc format
    #[must_use]
    pub fn quote_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.quote_time),
            QuoteResponse::Forex(x) => Some(x.quote.quote_time),
            QuoteResponse::Future(x) => Some(x.quote.quote_time),
            QuoteResponse::FutureOption(x) => Some(x.quote.quote_time),
            QuoteResponse::Index(_) | QuoteResponse::MutualFund(_) => None,
            QuoteResponse::Option(x) => Some(x.quote.quote_time),
        }
    }

    /// Returns the time of the last trade in Utc format
    #[must_use]
    pub fn trade_time(&self) -> chrono::DateTime<chrono::Utc> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => x.quote.trade_time,
            QuoteResponse::Forex(x) => x.quote.trade_time,
            QuoteResponse::Future(x) => x.quote.trade_time,
            QuoteResponse::FutureOption(x) => x.quote.trade_time,
            QuoteResponse::Index(x) => x.quote.trade_time,
            QuoteResponse::MutualFund(x) => x.quote.trade_time,
            QuoteResponse::Option(x) => x.quote.trade_time,
        }
    }

    /// Returns the total volume of trades for the day including pre/post market
    #[must_use]
    pub fn total_volume(&self) -> Option<u64> {
        match self {
            QuoteResponse::Bond(x) => unimplemented!("{x}"),
            QuoteResponse::Equity(x) => Some(x.quote.total_volume),
            QuoteResponse::Forex(x) => Some(x.quote.total_volume),
            QuoteResponse::Future(x) => Some(x.quote.total_volume),
            QuoteResponse::FutureOption(x) => Some(x.quote.total_volume),
            QuoteResponse::Index(x) => Some(x.quote.total_volume),
            QuoteResponse::MutualFund(x) => x.quote.total_volume,
            QuoteResponse::Option(x) => Some(x.quote.total_volume),
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_json_diff::{CompareMode, Config, NumericMode, assert_json_matches};
    use float_cmp::assert_approx_eq;
    use test_log::test;

    use super::*;
    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse.json"
        ));

        let val = serde_json::from_str::<HashMap<String, QuoteResponse>>(json);
        tracing::debug!(?val);
        assert!(val.is_ok());
    }

    #[test]
    fn test_serde_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse_real.json"
        ));
        let json: serde_json::Value = serde_json::from_str(json).unwrap();

        let val = serde_json::from_value::<QuoteResponseMap>(json.clone()).unwrap();
        tracing::debug!(?val);

        assert_json_matches!(
            val,
            json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat)
        );
    }

    #[test]
    fn test_methods() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse_real.json"
        ));
        let json: serde_json::Value = serde_json::from_str(json).unwrap();

        let mut val = serde_json::from_value::<QuoteResponseMap>(json.clone()).unwrap();

        let result = val.responses.remove("AAPL").unwrap();
        assert_eq!("AAPL", result.symbol());
        assert_approx_eq!(f64, 199.62, result.n52week_high().unwrap());
        assert_approx_eq!(f64, 164.075, result.n52week_low().unwrap());
        assert_approx_eq!(f64, 189.92, result.ask_price().unwrap());
        assert_eq!(1, result.ask_size().unwrap());
        assert_eq!(
            chrono::DateTime::from_timestamp_millis(1_715_990_363_904).unwrap(),
            result.ask_time().unwrap()
        );
        assert_approx_eq!(f64, 189.9, result.bid_price().unwrap());
        assert_eq!(6, result.bid_size().unwrap());
        assert_eq!(
            chrono::DateTime::from_timestamp_millis(1_715_990_363_904).unwrap(),
            result.bid_time().unwrap()
        );
        assert_approx_eq!(f64, 189.84, result.close_price());
        assert_approx_eq!(f64, 190.81, result.high_price().unwrap());
        assert_approx_eq!(f64, 189.9, result.last_price().unwrap());
        assert_eq!(2, result.last_size().unwrap());
        assert_approx_eq!(f64, 189.18, result.low_price().unwrap());
        assert_approx_eq!(f64, 0.06, result.net_change());
        assert_approx_eq!(f64, 189.51, result.open_price().unwrap());
        assert_eq!(
            chrono::DateTime::from_timestamp_millis(1_715_990_363_904).unwrap(),
            result.quote_time().unwrap()
        );
        assert_eq!(
            chrono::DateTime::from_timestamp_millis(1_715_990_395_834).unwrap(),
            result.trade_time()
        );
        assert_eq!(41_282_925, result.total_volume().unwrap());
    }
}
