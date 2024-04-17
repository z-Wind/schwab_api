use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewOrder {
    pub order_id: i64,
    pub order_strategy: OrderStrategy,
    pub order_validation_result: OrderValidationResult,
    pub commission_and_fee: CommissionAndFee,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStrategy {
    pub account_number: String,
    pub advanced_order_type: String,
    pub close_time: String,
    pub entered_time: String,
    pub order_balance: OrderBalance,
    pub order_strategy_type: String,
    pub order_version: i64,
    pub session: String,
    pub status: String,
    pub all_or_none: bool,
    pub discretionary: bool,
    pub duration: String,
    pub filled_quantity: i64,
    pub order_type: String,
    pub order_value: i64,
    pub price: i64,
    pub quantity: i64,
    pub remaining_quantity: i64,
    pub sell_non_marginable_first: bool,
    pub settlement_instruction: String,
    pub strategy: String,
    pub amount_indicator: String,
    pub order_legs: Vec<OrderLeg>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBalance {
    pub order_value: i64,
    pub projected_available_fund: i64,
    pub projected_buying_power: i64,
    pub projected_commission: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLeg {
    pub ask_price: i64,
    pub bid_price: i64,
    pub last_price: i64,
    pub mark_price: i64,
    pub projected_commission: i64,
    pub quantity: i64,
    pub final_symbol: String,
    pub leg_id: i64,
    pub asset_type: String,
    pub instruction: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderValidationResult {
    pub alerts: Vec<OrderValidationDetail>,
    pub accepts: Vec<OrderValidationDetail>,
    pub rejects: Vec<OrderValidationDetail>,
    pub reviews: Vec<OrderValidationDetail>,
    pub warns: Vec<OrderValidationDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderValidationDetail {
    pub validation_rule_name: String,
    pub message: String,
    pub activity_message: String,
    pub original_severity: String,
    pub override_name: String,
    pub override_severity: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionAndFee {
    pub commission: Commission,
    pub fee: Fee,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionValue {
    pub value: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
    pub fee_legs: Vec<FeeLeg>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeLeg {
    pub fee_values: Vec<FeeValue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeValue {
    pub value: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
