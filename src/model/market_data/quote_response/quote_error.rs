use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteError {
    pub invalid_cusips: Vec<String>,
    #[serde(rename = "invalidSSIDs")]
    pub invalid_ssids: Vec<String>,
    pub invalid_symbols: Vec<String>,
}
