use serde::Deserialize;
use serde::Serialize;

use crate::model::trader::accounts::AccountsInstrument;

use super::preview_order::Instruction;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub session: Session,
    pub duration: Duration,
    pub order_type: OrderType,
    pub cancel_time: chrono::DateTime<chrono::Utc>,
    pub complex_order_strategy_type: ComplexOrderStrategyType,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub remaining_quantity: f64,
    pub requested_destination: RequestedDestination,
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
    /// xml: OrderedMap { "name": "orderLegCollection", "wrapped": true }
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
    pub tag: String,
    pub account_number: i64,
    /// xml: OrderedMap { "name": "orderActivity", "wrapped": true }
    pub order_activity_collection: Vec<OrderActivity>,
    /// xml: OrderedMap { "name": "replacingOrder", "wrapped": true }
    pub replacing_order_collection: Vec<String>,
    /// xml: OrderedMap { "name": "childOrder", "wrapped": true }
    pub child_order_strategies: Vec<String>,
    pub status_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLegCollection {
    pub order_leg_type: OrderLegType,
    pub leg_id: i64,
    pub instrument: AccountsInstrument,
    pub instruction: Instruction,
    pub position_effect: PositionEffect,
    pub quantity: f64,
    pub quantity_type: QuantityType,
    pub div_cap_gains: DivCapGains,
    pub to_symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderActivity {
    pub activity_type: ActivityType,
    pub execution_type: ExecutionType,
    pub quantity: f64,
    pub order_remaining_quantity: f64,
    /// xml: OrderedMap { "name": "executionLegs", "wrapped": true }
    pub execution_legs: Vec<ExecutionLeg>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionLeg {
    pub leg_id: i64,
    pub price: f64,
    pub quantity: f64,
    pub mismarked_quantity: f64,
    pub instrument_id: i64,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Session {
    #[default]
    Normal,
    Am,
    Pm,
    Seamless,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Duration {
    #[default]
    Day,
    GoodTillCancel,
    FillOrKill,
    ImmediateOrCancel,
    EndOfWeek,
    EndOfMonth,
    NextEndOfMonth,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
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
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComplexOrderStrategyType {
    #[default]
    None,
    Covered,
    Vertical,
    BackRatio,
    Calendar,
    Diagonal,
    Straddle,
    Strangle,
    CollarSynthetic,
    Butterfly,
    Condor,
    IronCondor,
    VerticalRoll,
    CollarWithStock,
    DoubleDiagonal,
    UnbalancedButterfly,
    UnbalancedCondor,
    UnbalancedIronCondor,
    UnbalancedVerticalRoll,
    MutualFundSwap,
    Custom,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequestedDestination {
    #[default]
    Inet,
    EcnArca,
    Cboe,
    Amex,
    Phlx,
    Ise,
    Box,
    Nyse,
    Nasdaq,
    Bats,
    C2,
    Auto,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StopPriceLinkBasis {
    #[default]
    Manual,
    Base,
    Trigger,
    Last,
    Bid,
    Ask,
    AskBid,
    Mark,
    Average,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StopPriceLinkType {
    #[default]
    Value,
    Percent,
    Tick,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StopType {
    #[default]
    Standard,
    Bid,
    Ask,
    Last,
    Mark,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceLinkBasis {
    #[default]
    Manual,
    Base,
    Trigger,
    Last,
    Bid,
    Ask,
    AskBid,
    Mark,
    Average,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceLinkType {
    #[default]
    Value,
    Percent,
    Tick,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxLotMethod {
    #[default]
    Fifo,
    Lifo,
    HighCost,
    LowCost,
    AverageCost,
    SpecificLot,
    LossHarvester,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpecialInstruction {
    #[default]
    AllOrNone,
    DoNotReduce,
    AllOrNoneDoNotReduce,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStrategyType {
    #[default]
    Single,
    Cancel,
    Recall,
    Pair,
    Flatten,
    TwoDaySwap,
    BlastAll,
    Oco,
    Trigger,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
    #[default]
    Execution,
    OrderAction,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionType {
    #[default]
    Fill,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderLegType {
    #[default]
    Equity,
    Option,
    Index,
    MutualFund,
    CashEquivalent,
    FixedIncome,
    Currency,
    CollectiveInvestment,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionEffect {
    #[default]
    Opening,
    Closing,
    Automatic,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuantityType {
    #[default]
    AllShares,
    Dollars,
    Shares,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DivCapGains {
    #[default]
    Reinvest,
    Payout,
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
