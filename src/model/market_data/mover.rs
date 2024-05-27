use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mover {
    pub screeners: Vec<Screener>,
}

/// Security info of most moved with in an index
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Screener {
    /// percent or value changed, by default its percent changed
    pub change: Option<f64>,
    /// Name of security
    pub description: String,
    pub direction: Option<Direction>,
    /// what was last quoted price
    pub last: Option<f64>,
    /// schwab security symbol
    pub symbol: String,
    pub total_volume: u64,

    // not in schema
    pub volume: Option<u64>,
    pub last_price: Option<f64>,
    pub net_change: Option<f64>,
    pub market_share: Option<f64>,
    pub trades: Option<i64>,
    pub net_percent_change: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
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

    #[test]
    fn test_de_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/Mover_real.json"
        ));

        let val = serde_json::from_str::<Mover>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
