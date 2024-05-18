use serde::Deserialize;
use serde::Serialize;

use super::order::ComplexOrderStrategyType;
use super::order::Duration;
use super::order::OrderActivity;
use super::order::OrderLegCollection;
use super::order::OrderStrategyType;
use super::order::PriceLinkBasis;
use super::order::PriceLinkType;
use super::order::Session;
use super::order::SpecialInstruction;
use super::order::Status;
use super::order::StopPriceLinkBasis;
use super::order::StopPriceLinkType;
use super::order::StopType;
use super::order::TaxLotMethod;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub session: Session,
    pub duration: Duration,
    pub order_type: OrderTypeRequest,
    pub cancel_time: chrono::DateTime<chrono::Utc>,
    pub complex_order_strategy_type: ComplexOrderStrategyType,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub remaining_quantity: f64,
    pub destination_link_name: String,
    pub release_time: chrono::DateTime<chrono::Utc>,
    pub stop_price: f64,
    pub stop_price_link_basis: StopPriceLinkBasis,
    pub stop_price_link_type: StopPriceLinkType,
    pub stop_price_offset: f64,
    pub stop_type: StopType,
    pub price_link_basis: PriceLinkBasis,
    pub price_link_type: PriceLinkType,
    pub price: f64,
    pub tax_lot_method: TaxLotMethod,
    /// xml: `OrderedMap` { "name": "orderLegCollection", "wrapped": true }
    pub order_leg_collection: Vec<OrderLegCollection>,
    pub activation_price: f64,
    pub special_instruction: SpecialInstruction,
    pub order_strategy_type: OrderStrategyType,
    pub order_id: i64,
    /// default: false
    pub cancelable: bool,
    /// default: false
    pub editable: bool,
    pub status: Status,
    pub entered_time: chrono::DateTime<chrono::Utc>,
    pub close_time: chrono::DateTime<chrono::Utc>,
    pub account_number: i64,
    /// xml: `OrderedMap` { "name": "orderActivity", "wrapped": true }
    pub order_activity_collection: Vec<OrderActivity>,
    /// xml: `OrderedMap` { "name": "replacingOrder", "wrapped": true }
    pub replacing_order_collection: Vec<String>,
    /// xml: `OrderedMap` { "name": "childOrder", "wrapped": true }
    pub child_order_strategies: Vec<String>,
    pub status_description: String,
}

/// Same as `super::order::OrderType`, but does not have UNKNOWN since this type is not allowed as an input
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderTypeRequest {
    #[default]
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
    Cabinet,
    NonMarketable,
    MarketOnClose,
    Exercise,
    TrailingStopLimit,
    NetDebit,
    NetCredit,
    NetZero,
    LimitOnClose,
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
