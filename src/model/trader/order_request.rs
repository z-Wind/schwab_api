use serde::Deserialize;
use serde::Serialize;

use crate::model::trader::order::OrderActivity;
use crate::model::trader::order::OrderLegCollection;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub session: String,
    pub duration: String,
    pub order_type: String,
    pub cancel_time: String,
    pub complex_order_strategy_type: String,
    pub quantity: i64,
    pub filled_quantity: i64,
    pub remaining_quantity: i64,
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
    pub account_number: i64,
    pub order_activity_collection: Vec<OrderActivity>,
    pub replacing_order_collection: Vec<String>,
    pub child_order_strategies: Vec<String>,
    pub status_description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/OrderRequest.json"
        ));

        let val = serde_json::from_str::<OrderRequest>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}