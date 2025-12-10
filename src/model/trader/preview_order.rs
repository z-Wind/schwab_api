use serde::Deserialize;
use serde::Serialize;

use super::accounts::AssetType;
use super::order::ComplexOrderStrategyType;
use super::order::Duration;
use super::order::OrderStrategyType;
use super::order::OrderType;
use super::order::PositionEffect;
use super::order::Session;
use super::transactions::TransactionInstrument;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewOrder {
    pub order_id: i64,
    pub order_strategy: OrderStrategy,
    pub order_validation_result: OrderValidationResult,
    pub commission_and_fee: CommissionAndFee,
}

#[allow(clippy::struct_field_names)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStrategy {
    pub account_number: String,
    pub advanced_order_type: AdvancedOrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_time: Option<chrono::DateTime<chrono::Utc>>,
    pub entered_time: chrono::DateTime<chrono::Utc>,
    pub order_balance: OrderBalance,
    pub order_strategy_type: OrderStrategyType,
    pub order_version: i64,
    pub session: Session,
    pub status: APIOrderStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_or_none: Option<bool>,
    pub discretionary: bool,
    pub duration: Duration,
    pub filled_quantity: f64,
    pub order_type: OrderType,
    pub order_value: f64,
    pub price: f64,
    pub quantity: f64,
    pub remaining_quantity: f64,
    pub sell_non_marginable_first: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_instruction: Option<SettlementInstruction>,
    pub strategy: ComplexOrderStrategyType,
    pub amount_indicator: AmountIndicator,
    pub order_legs: Vec<OrderLeg>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBalance {
    pub order_value: f64,
    pub projected_available_fund: f64,
    pub projected_buying_power: f64,
    pub projected_commission: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLeg {
    pub ask_price: f64,
    pub bid_price: f64,
    pub last_price: f64,
    pub mark_price: f64,
    pub projected_commission: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_symbol: Option<String>,
    pub leg_id: i64,
    pub asset_type: AssetType,
    pub instruction: Instruction,

    // not in schema
    pub instrument: Option<TransactionInstrument>,
    pub position_effect: Option<PositionEffect>,
}
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderValidationResult {
    pub alerts: Option<Vec<OrderValidationDetail>>,
    pub accepts: Option<Vec<OrderValidationDetail>>,
    pub rejects: Option<Vec<OrderValidationDetail>>,
    pub reviews: Option<Vec<OrderValidationDetail>>,
    pub warns: Option<Vec<OrderValidationDetail>>,
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderValidationDetail {
    pub validation_rule_name: Option<String>,
    pub message: Option<String>,
    pub activity_message: String,
    pub original_severity: APIRuleAction,
    pub override_name: Option<String>,
    pub override_severity: Option<APIRuleAction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionAndFee {
    pub commission: Commission,
    pub fee: Fees,
    pub true_commission: Commission,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commission {
    pub commission_legs: Vec<CommissionLeg>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionLeg {
    pub commission_values: Vec<CommissionValue>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionValue {
    pub value: f64,
    #[serde(rename = "type")]
    pub type_field: FeeType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fees {
    pub fee_legs: Vec<FeeLeg>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeLeg {
    pub fee_values: Vec<FeeValue>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeValue {
    pub value: f64,
    #[serde(rename = "type")]
    pub type_field: FeeType,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AmountIndicator {
    #[default]
    Dollars,
    Shares,
    AllShares,
    Percentage,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SettlementInstruction {
    #[default]
    Regular,
    Cash,
    NextDay,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdvancedOrderType {
    #[default]
    None,
    Oto,
    Oco,
    Otoco,
    Ot2oco,
    Ot3oco,
    BlastAll,
    Ota,
    Pair,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum APIOrderStatus {
    #[default]
    AwaitingParentOrder,
    AwaitingCondition,
    AwaitingStopCondition,
    AwaitingManualReview,
    Accepted,
    AwaitingUrOut,
    PendingActivation,
    Queued,
    Working,
    Rejected,
    PendingCancel,
    Canceled,
    PendingReplace,
    Replaced,
    Filled,
    Expired,
    New,
    AwaitingReleaseTime,
    PendingAcknowledgement,
    PendingRecall,
    Unknown,
}

/// Instructions for opening and closing equity positions.
/// Instructions for opening and closing options positions.
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Instruction {
    #[default]
    /// Open a long equity position
    Buy,
    /// Close a long equity position
    Sell,
    /// Close a short equity position
    BuyToCover,
    /// Open a short equity position
    SellShort,

    /// Enter a new long option position
    BuyToOpen,
    /// Exit an existing short position in an option
    BuyToClose,
    /// Enter a short position in an option
    SellToOpen,
    /// Exit an existing long option position
    SellToClose,

    Exchange,
    SellShortExempt,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum APIRuleAction {
    #[default]
    Accept,
    Alert,
    Reject,
    Review,
    Warn,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeeType {
    #[default]
    Commission,
    SecFee,
    StrFee,
    RFee,
    CdscFee,
    OptRegFee,
    AdditionalFee,
    MiscellaneousFee,
    Ftt,
    FuturesClearingFee,
    FuturesDeskOfficeFee,
    FuturesExchangeFee,
    FuturesGlobexFee,
    FuturesNfaFee,
    FuturesPitBrokerageFee,
    FuturesTransactionFee,
    LowProceedsCommission,
    BaseCharge,
    GeneralCharge,
    GstFee,
    TafFee,
    IndexOptionFee,
    TefraTax,
    StateTax,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::{CompareMode, Config, NumericMode, assert_json_matches};

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/PreviewOrder.json"
        ));

        let val = serde_json::from_str::<PreviewOrder>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_serde_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/PreviewOrder_real.json"
        ));
        let json: serde_json::Value = serde_json::from_str(json).unwrap();

        let val = serde_json::from_value::<PreviewOrder>(json.clone()).unwrap();
        dbg!(&val);

        assert_json_matches!(
            val,
            json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat)
        );
    }

    #[test]
    fn test_serde_real2() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/PreviewOrder_real2.json"
        ));
        let json: serde_json::Value = serde_json::from_str(json).unwrap();

        let val = serde_json::from_value::<PreviewOrder>(json.clone()).unwrap();
        dbg!(&val);

        assert_json_matches!(
            val,
            json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat)
        );
    }
}
