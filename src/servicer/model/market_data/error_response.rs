use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub errors: Vec<Error>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub id: String,
    pub status: String,
    pub title: String,
    pub detail: Option<String>,
    pub source: Option<Source>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub pointer: Option<Vec<String>>,
    pub parameter: Option<String>,
    pub header: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/ErrorResponse.json"
        ));

        let val = serde_json::from_str::<ErrorResponse>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}