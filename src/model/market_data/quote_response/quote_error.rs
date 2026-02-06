use serde::Deserialize;
use serde::Serialize;

/// Partial or Custom errors per request
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteError {
    /// list of invalid cusips from request
    pub invalid_cusips: Option<Vec<String>>,

    /// list of invalid SSIDs from request
    #[serde(rename = "invalidSSIDs")]
    pub invalid_ssids: Option<Vec<String>>,

    ///list of invalid symbols from request
    pub invalid_symbols: Option<Vec<String>>,
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
            "/tests/model/MarketData/QuoteResponse/QuoteError.json"
        ));

        let val = serde_json::from_str::<HashMap<String, QuoteError>>(json);
        tracing::debug!(?val);
        assert!(val.is_ok());
    }
}
