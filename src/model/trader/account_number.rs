use serde::Deserialize;
use serde::Serialize;

pub type AccountNumbers = Vec<AccountNumberHash>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountNumberHash {
    pub account_number: String,
    pub hash_value: String,
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/AccountNumbers.json"
        ));

        let val = serde_json::from_str::<AccountNumbers>(json);
        tracing::debug!(?val);
        assert!(val.is_ok());
    }
}
