use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpirationChain {
    pub expiration_list: Vec<Expiration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expiration {
    pub days_to_expiration: i64,
    pub expiration: Option<String>,
    pub expiration_type: String,
    pub standard: bool,
    pub settlement_type: Option<String>,
    pub option_roots: Option<String>,
    // not in schama
    pub expiration_date: chrono::NaiveDate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/ExpirationChain.json"
        ));

        let val = serde_json::from_str::<ExpirationChain>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
