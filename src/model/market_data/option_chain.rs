use serde::Deserialize;
use serde::Serialize;
use serde_with::{TimestampMilliSeconds, serde_as};
use std::collections::HashMap;

use super::quote_response::option::ExerciseType;
use super::quote_response::option::ExpirationType;
use super::quote_response::option::SettlementType;
use crate::Number;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChain {
    pub symbol: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<Underlying>,
    pub strategy: Strategy,
    pub interval: Number,
    pub is_delayed: bool,
    pub is_index: bool,
    pub days_to_expiration: Number,
    pub interest_rate: Number,
    pub underlying_price: Number,
    pub volatility: Number,
    pub call_exp_date_map: HashMap<String, HashMap<String, Vec<OptionContract>>>,
    pub put_exp_date_map: HashMap<String, HashMap<String, Vec<OptionContract>>>,

    // Fields not explicitly defined in the official schema
    // ===================================================
    pub number_of_contracts: Option<i64>,
    pub asset_main_type: Option<String>,
    pub asset_sub_type: Option<String>,
    pub is_chain_truncated: Option<bool>,
    // ===================================================
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Underlying {
    pub ask: Number,
    pub ask_size: i64,
    pub bid: Number,
    pub bid_size: i64,
    pub change: Number,
    pub close: Number,
    pub delayed: bool,
    pub description: String,
    pub exchange_name: ExchangeName,
    pub fifty_two_week_high: Number,
    pub fifty_two_week_low: Number,
    pub high_price: Number,
    pub last: Number,
    pub low_price: Number,
    pub mark: Number,
    pub mark_change: Number,
    pub mark_percent_change: Number,
    pub open_price: Number,
    pub percent_change: Number,
    pub quote_time: i64,
    pub symbol: String,
    pub total_volume: u64,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,
}

#[serde_as]
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionContract {
    pub put_call: PutCall,
    pub symbol: String,
    pub description: String,
    pub exchange_name: String,
    pub bid_price: Option<Number>,
    pub ask_price: Option<Number>,
    pub last_price: Option<Number>,
    pub mark_price: Option<Number>,
    pub bid_size: i64,
    pub ask_size: i64,
    pub last_size: i64,
    pub high_price: Number,
    pub low_price: Number,
    pub open_price: Number,
    pub close_price: Number,
    pub total_volume: u64,
    #[serde_as(as = "Option<TimestampMilliSeconds<i64>>")]
    pub trade_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub quote_time_in_long: chrono::DateTime<chrono::Utc>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time_in_long: chrono::DateTime<chrono::Utc>,
    pub net_change: Number,
    pub volatility: Number,
    pub delta: Number,
    pub gamma: Number,
    pub theta: Number,
    pub vega: Number,
    pub rho: Number,
    pub time_value: Number,
    pub open_interest: i64,
    pub is_in_the_money: Option<bool>,
    pub theoretical_option_value: Number,
    pub theoretical_volatility: Number,
    pub is_mini: Option<bool>,
    pub is_non_standard: Option<bool>,
    pub option_deliverables_list: Vec<OptionDeliverable>,
    pub strike_price: Number,
    pub expiration_date: chrono::DateTime<chrono::FixedOffset>,
    pub days_to_expiration: i64,
    pub expiration_type: ExpirationType,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub last_trading_day: chrono::DateTime<chrono::Utc>,
    pub multiplier: Number,
    pub settlement_type: SettlementType,
    pub deliverable_note: String,
    pub is_index_option: Option<bool>,
    pub percent_change: Number,
    pub mark_change: Number,
    pub mark_percent_change: Number,
    pub is_penny_pilot: Option<bool>,
    pub intrinsic_value: Number,
    pub option_root: String,

    // Fields not explicitly defined in the official schema
    // ===================================================
    pub bid: Option<Number>,
    pub ask: Option<Number>,
    pub last: Option<Number>,
    pub mark: Option<Number>,
    pub bid_ask_size: Option<String>,
    pub exercise_type: Option<ExerciseType>,
    pub high_52_week: Option<Number>,
    pub low_52_week: Option<Number>,
    pub extrinsic_value: Option<Number>,
    pub in_the_money: Option<bool>,
    pub mini: Option<bool>,
    pub non_standard: Option<bool>,
    pub penny_pilot: Option<bool>,
    // ===================================================
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionDeliverable {
    pub symbol: String,
    pub asset_type: String,
    pub deliverable_units: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(other)]
    Unknown,
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
    use assert_json_diff::{CompareMode, Config, NumericMode, assert_json_matches};
    use regex::Regex;
    use test_log::test;

    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/OptionChain.json"
        ));

        let val = serde_json::from_str::<OptionChain>(json);
        tracing::debug!(?val);
        assert!(val.is_ok(), "Failed to deserialize: {:?}", val.err());
    }

    #[test]
    fn test_serde_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/OptionChain_real.json"
        ));

        // 1. Create a more robust regex to handle multiple ISO 8601 variations:
        // - (\.000)? : Optional milliseconds
        // - (\+00:00|\+0000) : Timezone as +00:00 or +0000
        let re = Regex::new(r"(\.000)?(\+00:00|\+0000)").unwrap();

        // 2. Normalize all variations to "Z" to match Rust's default output
        let json = re.replace_all(json, "Z");

        let json: serde_json::Value = serde_json::from_str(&json).unwrap();

        let val = serde_json::from_value::<OptionChain>(json.clone()).unwrap();
        // tracing::debug!(?val);

        assert_json_matches!(
            val,
            json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat)
        );
    }
}
