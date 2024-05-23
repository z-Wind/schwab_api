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

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse.json"
        ));

        let val = serde_json::from_str::<HashMap<String, QuoteResponse>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_de_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse_real.json"
        ));

        let val = serde_json::from_str::<QuoteResponseMap>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
