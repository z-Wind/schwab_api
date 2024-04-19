use serde::Deserialize;
use serde::Serialize;

pub type AccountNumbers = Vec<AccountNumber>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountNumber {
    pub account_number: String,
    pub hash_value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/AccountNumbers.json"
        ));

        let val = serde_json::from_str::<AccountNumbers>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
