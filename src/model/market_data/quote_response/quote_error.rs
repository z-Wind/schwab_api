use serde::Deserialize;
use serde::Serialize;

/// Partial or Custom errors per request
#[allow(clippy::struct_field_names)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteError {
    /// list of invalid cusips from request
    pub invalid_cusips: Vec<String>,

    /// list of invalid SSIDs from request
    #[serde(rename = "invalidSSIDs")]
    pub invalid_ssids: Vec<String>,

    ///list of invalid symbols from request
    pub invalid_symbols: Vec<String>,
}
