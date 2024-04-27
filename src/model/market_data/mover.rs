use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mover {
    pub screeners: Vec<Screener>,
}

/// Security info of most moved with in an index
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Screener {
    /// percent or value changed, by default its percent changed
    pub change: f64,
    /// Name of security
    pub description: String,
    pub direction: Direction,
    /// what was last quoted price
    pub last: f64,
    /// schwab security symbol
    pub symbol: String,
    pub total_volume: i64,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    #[default]
    Up,
    Down,
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
