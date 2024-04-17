use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

pub type Markets = HashMap<String, HashMap<String, Hours>>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hours {
    pub date: String,
    pub market_type: String,
    pub exchange: Option<String>,
    pub category: Option<String>,
    pub product: String,
    pub product_name: String,
    pub is_open: bool,
    pub session_hours: HashMap<String, Vec<Interval>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    pub start: String,
    pub end: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/Markets.json"
        ));

        let val = serde_json::from_str::<Markets>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
