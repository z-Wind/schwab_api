use serde::Deserialize;
use serde::Serialize;

use crate::model::trader::accounts::AccountsInstrument;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub session: String,
    pub duration: String,
    pub order_type: String,
    pub cancel_time: String,
    pub complex_order_strategy_type: String,
    pub quantity: i64,
    pub filled_quantity: i64,
    pub remaining_quantity: i64,
    pub requested_destination: String,
    pub destination_link_name: String,
    pub release_time: String,
    pub stop_price: i64,
    pub stop_price_link_basis: String,
    pub stop_price_link_type: String,
    pub stop_price_offset: i64,
    pub stop_type: String,
    pub price_link_basis: String,
    pub price_link_type: String,
    pub price: i64,
    pub tax_lot_method: String,
    pub order_leg_collection: Vec<OrderLegCollection>,
    pub activation_price: i64,
    pub special_instruction: String,
    pub order_strategy_type: String,
    pub order_id: i64,
    pub cancelable: bool,
    pub editable: bool,
    pub status: String,
    pub entered_time: String,
    pub close_time: String,
    pub tag: String,
    pub account_number: i64,
    pub order_activity_collection: Vec<OrderActivity>,
    pub replacing_order_collection: Vec<String>,
    pub child_order_strategies: Vec<String>,
    pub status_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLegCollection {
    pub order_leg_type: String,
    pub leg_id: i64,
    pub instrument: AccountsInstrument,
    pub instruction: String,
    pub position_effect: String,
    pub quantity: i64,
    pub quantity_type: String,
    pub div_cap_gains: String,
    pub to_symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderActivity {
    pub activity_type: String,
    pub execution_type: String,
    pub quantity: i64,
    pub order_remaining_quantity: i64,
    pub execution_legs: Vec<ExecutionLeg>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionLeg {
    pub leg_id: i64,
    pub price: i64,
    pub quantity: i64,
    pub mismarked_quantity: i64,
    pub instrument_id: i64,
    pub time: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_order() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Order.json"
        ));

        let val = serde_json::from_str::<Order>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_de_orders() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Orders.json"
        ));

        let val = serde_json::from_str::<Vec<Order>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
