use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mover {
    pub screeners: Vec<Screener>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Screener {
    pub change: i64,
    pub description: String,
    pub direction: String,
    pub last: i64,
    pub symbol: String,
    pub total_volume: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/Mover.json"
        ));

        let val = serde_json::from_str::<Mover>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}