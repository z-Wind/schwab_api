use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use super::quote_response::option::ExpirationType;
use super::quote_response::option::SettlementType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChain {
    pub symbol: String,
    pub status: String,
    pub underlying: Option<Underlying>,
    pub strategy: Strategy,
    pub interval: f64,
    pub is_delayed: bool,
    pub is_index: bool,
    pub days_to_expiration: f64,
    pub interest_rate: f64,
    pub underlying_price: f64,
    pub volatility: f64,
    pub call_exp_date_map: HashMap<String, HashMap<String, Vec<OptionContract>>>,
    pub put_exp_date_map: HashMap<String, HashMap<String, Vec<OptionContract>>>,

    // not in schema
    pub number_of_contracts: Option<i64>,
    pub asset_main_type: Option<String>,
    pub asset_sub_type: Option<String>,
    pub is_chain_truncated: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Underlying {
    pub ask: i64,
    pub ask_size: i64,
    pub bid: i64,
    pub bid_size: i64,
    pub change: i64,
    pub close: i64,
    pub delayed: bool,
    pub description: String,
    pub exchange_name: ExchangeName,
    pub fifty_two_week_high: i64,
    pub fifty_two_week_low: i64,
    pub high_price: f64,
    pub last: i64,
    pub low_price: f64,
    pub mark: i64,
    pub mark_change: f64,
    pub mark_percent_change: f64,
    pub open_price: f64,
    pub percent_change: f64,
    pub quote_time: i64,
    pub symbol: String,
    pub total_volume: i64,
    pub trade_time: i64,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionContract {
    pub put_call: PutCall,
    pub symbol: String,
    pub description: String,
    pub exchange_name: String,
    pub bid_price: Option<f64>,
    pub ask_price: Option<f64>,
    pub last_price: Option<f64>,
    pub mark_price: Option<f64>,
    pub bid_size: i64,
    pub ask_size: i64,
    pub last_size: i64,
    pub high_price: f64,
    pub low_price: f64,
    pub open_price: f64,
    pub close_price: f64,
    pub total_volume: i64,
    pub trade_date: Option<i64>,
    pub quote_time_in_long: i64,
    pub trade_time_in_long: i64,
    pub net_change: f64,
    pub volatility: f64,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub time_value: f64,
    pub open_interest: i64,
    pub is_in_the_money: Option<bool>,
    pub theoretical_option_value: f64,
    pub theoretical_volatility: f64,
    pub is_mini: Option<bool>,
    pub is_non_standard: Option<bool>,
    pub option_deliverables_list: Vec<OptionDeliverable>,
    pub strike_price: f64,
    pub expiration_date: String,
    pub days_to_expiration: i64,
    pub expiration_type: ExpirationType,
    pub last_trading_day: i64,
    pub multiplier: f64,
    pub settlement_type: SettlementType,
    pub deliverable_note: String,
    pub is_index_option: Option<bool>,
    pub percent_change: f64,
    pub mark_change: f64,
    pub mark_percent_change: f64,
    pub is_penny_pilot: Option<bool>,
    pub intrinsic_value: f64,
    pub option_root: String,

    // not in schema
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub last: Option<f64>,
    pub mark: Option<f64>,
    pub extrinsic_value: Option<f64>,
    pub in_the_money: Option<bool>,
    pub mini: Option<bool>,
    pub non_standard: Option<bool>,
    pub penny_pilot: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionDeliverable {
    pub symbol: String,
    pub asset_type: String,
    pub deliverable_units: f64,
    pub currency_type: Option<String>,
}

/// Available values : `SINGLE`, `ANALYTICAL`, `COVERED`, `VERTICAL`, `CALENDAR`, `STRANGLE`, `STRADDLE`, `BUTTERFLY`, `CONDOR`, `DIAGONAL`, `COLLAR`, `ROLL`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Strategy {
    Single,
    Analytical,
    Covered,
    Vertical,
    Calendar,
    Strangle,
    Straddle,
    Butterfly,
    Condor,
    Diagonal,
    Collar,
    Roll,
}

/// Available values : IND, ASE, NYS, NAS, NAP, PAC, OPR, BATS
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ExchangeName {
    Ind,
    Ase,
    Nys,
    Nas,
    Nap,
    Pac,
    Opr,
    Bats,
}

/// Available values : `PUT`, `CALL`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PutCall {
    Put,
    Call,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/OptionChain.json"
        ));

        let val = serde_json::from_str::<OptionChain>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_de_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/OptionChain_real.json"
        ));

        let val = serde_json::from_str::<OptionChain>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
