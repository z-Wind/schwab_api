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

use crate::api::parameter::QuoteField;

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

/// Request one or more quote data in POST body
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuoteRequest {
    /// example: List [ 808524680, 594918104 ]
    ///
    /// List of cusip, max of 500 of symbols+cusip+ssids
    cusips: Vec<String>,

    /// example: quote,reference
    ///
    /// comma separated list of nodes in each quote
    ///
    /// possible values are quote,fundamental,reference,extended,regular. Dont send this attribute for full response.
    fields: Vec<QuoteField>,

    /// example: List [ 1516105793, 34621523 ]
    ///
    ///List of Schwab securityid[SSID], max of 500 of symbols+cusip+ssids
    ssids: Vec<i64>,

    /// example: List [ `MRAD`, `EATOF`, `EBIZ`, `AAPL`, `BAC`, `AAAHX`, `AAAIX`, `$DJI`, `$SPX`, `MVEN`, `SOBS`, `TOITF`, `CNSWF`, `AMZN 230317C01360000`, `DJX 231215C00290000`, `/ESH23`, `./ADUF23C0.55`, `AUD/CAD` ]
    ///
    /// List of symbols, max of 500 of symbols+cusip+ssids
    symbols: Vec<String>,

    /// example: true
    ///
    /// Get realtime quotes and skip entitlement check
    realtime: bool,

    /// example: true
    ///
    /// Include indicative symbol quotes for all ETF symbols in request. If ETF symbol ABC is in request and indicative=true API will return quotes for ABC and its corresponding indicative quote for $ABC.IV
    indicative: bool,
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
    fn test_de2() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse_real.json"
        ));

        let val = serde_json::from_str::<QuoteResponseMap>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
