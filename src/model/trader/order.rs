use serde::Deserialize;
use serde::Serialize;

use crate::model::trader::accounts::AccountsInstrument;

use super::preview_order::Instruction;

#[allow(clippy::struct_field_names)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub session: Session,
    pub duration: Duration,
    pub order_type: OrderType,
    pub cancel_time: Option<chrono::DateTime<chrono::Utc>>,
    pub complex_order_strategy_type: ComplexOrderStrategyType,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub remaining_quantity: f64,
    pub requested_destination: RequestedDestination,
    pub destination_link_name: String,
    pub release_time: Option<chrono::DateTime<chrono::Utc>>,
    pub stop_price: Option<f64>,
    pub stop_price_link_basis: Option<StopPriceLinkBasis>,
    pub stop_price_link_type: Option<StopPriceLinkType>,
    pub stop_price_offset: Option<f64>,
    pub stop_type: Option<StopType>,
    pub price_link_basis: Option<PriceLinkBasis>,
    pub price_link_type: Option<PriceLinkType>,
    pub price: f64,
    pub tax_lot_method: Option<TaxLotMethod>,
    /// xml: `OrderedMap` { "name": "orderLegCollection", "wrapped": true }
    pub order_leg_collection: Vec<OrderLegCollection>,
    pub activation_price: Option<f64>,
    pub special_instruction: Option<SpecialInstruction>,
    pub order_strategy_type: OrderStrategyType,
    pub order_id: i64,
    /// default: false
    pub cancelable: bool,
    /// default: false
    pub editable: bool,
    pub status: Status,
    pub entered_time: chrono::DateTime<chrono::Utc>,
    pub close_time: Option<chrono::DateTime<chrono::Utc>>,
    pub tag: Option<String>,
    pub account_number: i64,
    /// xml: `OrderedMap` { "name": "orderActivity", "wrapped": true }
    pub order_activity_collection: Option<Vec<OrderActivity>>,
    /// xml: `OrderedMap` { "name": "replacingOrder", "wrapped": true }
    pub replacing_order_collection: Option<Vec<String>>,
    /// xml: `OrderedMap` { "name": "childOrder", "wrapped": true }
    pub child_order_strategies: Option<Vec<Order>>,
    pub status_description: Option<String>,
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
    pub quantity_type: Option<QuantityType>,
    pub div_cap_gains: Option<DivCapGains>,
    pub to_symbol: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderActivity {
    pub activity_type: ActivityType,
    pub execution_type: ExecutionType,
    pub quantity: f64,
    pub order_remaining_quantity: f64,
    /// xml: `OrderedMap` { "name": "executionLegs", "wrapped": true }
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

/// The market session during which the order trade should be executed.
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Session {
    /// Normal market hours, from 9:30am to 4:00pm Eastern.
    #[default]
    Normal,
    /// Premarket session, from 8:00am to 9:30am Eastern.
    Am,
    /// After-market session, from 4:00pm to 8:00pm Eastern.
    Pm,
    /// Orders are active during all trading sessions except the overnight
    /// session. This is the union of ``NORMAL``, ``AM``, and ``PM``.
    Seamless,
}

/// Length of time over which the trade will be active.
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Duration {
    /// Cancel the trade at the end of the trading day. Note if the order cannot
    /// be filled all at once, you may see partial executions throughout the day.
    #[default]
    Day,
    /// Keep the trade open for six months, or until the end of the cancel date,
    /// whichever is shorter. Note if the order cannot be filled all at once, you
    /// may see partial executions over the lifetime of the order.
    GoodTillCancel,
    /// Either execute the order immediately at the specified price, or cancel it
    /// immediately.
    FillOrKill,
    ImmediateOrCancel,
    EndOfWeek,
    EndOfMonth,
    NextEndOfMonth,
    Unknown,
}

/// Type of order to place.
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    #[default]
    /// Execute the order immediately at the best-available price.
    /// `More Info <https://www.investopedia.com/terms/m/marketorder.asp>`__.
    Market,
    /// Execute the order at your price or better.
    /// `More info <https://www.investopedia.com/terms/l/limitorder.asp>`__.
    Limit,
    /// Wait until the price reaches the stop price, and then immediately place a
    /// market order.
    /// `More Info <https://www.investopedia.com/terms/l/limitorder.asp>`__.
    Stop,
    /// Wait until the price reaches the stop price, and then immediately place a
    /// limit order at the specified price.
    /// `More Info <https://www.investopedia.com/terms/s/stop-limitorder.asp>`__.
    StopLimit,
    /// Similar to ``STOP``, except if the price moves in your favor, the stop
    /// price is adjusted in that direction. Places a market order if the stop
    /// condition is met.
    /// `More info <https://www.investopedia.com/terms/t/trailingstop.asp>`__.
    TrailingStop,
    Cabinet,
    NonMarketable,
    /// Place the order at the closing price immediately upon market close.
    /// `More info <https://www.investopedia.com/terms/m/marketonclose.asp>`__
    MarketOnClose,
    /// Exercise an option.
    Exercise,
    /// Similar to ``STOP_LIMIT``, except if the price moves in your favor, the
    /// stop price is adjusted in that direction. Places a limit order at the
    /// specified price if the stop condition is met.
    /// `More info <https://www.investopedia.com/terms/t/trailingstop.asp>`__.
    TrailingStopLimit,
    /// Place an order for an options spread resulting in a net debit.
    /// `More info <https://www.investopedia.com/ask/answers/042215/whats-difference-between-credit-spread-and-debt-spread.asp>`__
    NetDebit,
    /// Place an order for an options spread resulting in a net credit.
    /// `More info <https://www.investopedia.com/ask/answers/042215/whats-difference-between-credit-spread-and-debt-spread.asp>`__
    NetCredit,
    /// Place an order for an options spread resulting in neither a credit nor a
    /// debit.
    /// `More info <https://www.investopedia.com/ask/answers/042215/whats-difference-between-credit-spread-and-debt-spread.asp>`__
    NetZero,
    LimitOnClose,
    Unknown,
}

/// Explicit order strategies for executing multi-leg options orders.
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComplexOrderStrategyType {
    #[default]
    /// No complex order strategy. This is the default.
    None,
    /// `Covered call <https://tickertape.tdameritrade.com/trading/selling-covered-call-options-strategy-income-hedging-15135>`__
    Covered,
    /// `Vertical spread <https://tickertape.tdameritrade.com/trading/vertical-credit-spreads-high-probability-15846>`__
    Vertical,
    /// `Ratio backspread <https://tickertape.tdameritrade.com/trading/pricey-stocks-ratio-spreads-15306>`__
    BackRatio,
    /// `Calendar spread <https://tickertape.tdameritrade.com/trading/calendar-spreads-trading-primer-15095>`__
    Calendar,
    /// `Diagonal spread <https://tickertape.tdameritrade.com/trading/love-your-diagonal-spread-15030>`__
    Diagonal,
    /// `Straddle spread <https://tickertape.tdameritrade.com/trading/straddle-strangle-option-volatility-16208>`__
    Straddle,
    /// `Strandle spread <https://tickertape.tdameritrade.com/trading/straddle-strangle-option-volatility-16208>`__
    Strangle,
    CollarSynthetic,
    /// `Butterfly spread <https://tickertape.tdameritrade.com/trading/butterfly-spread-options-15976>`__
    Butterfly,
    /// `Condor spread <https://www.investopedia.com/terms/c/condorspread.asp>`__
    Condor,
    /// `Iron condor spread <https://tickertape.tdameritrade.com/trading/iron-condor-options-spread-your-trading-wings-15948>`__
    IronCondor,
    /// `Roll a vertical spread <https://tickertape.tdameritrade.com/trading/exit-winning-losing-trades-16685>`__
    VerticalRoll,
    /// `Collar strategy <https://tickertape.tdameritrade.com/trading/stock-hedge-options-collars-15529>`__
    CollarWithStock,
    /// `Double diagonal spread <https://optionstradingiq.com/the-ultimate-guide-to-double-diagonal-spreads/>`__
    DoubleDiagonal,
    /// `Unbalanced butterfy spread  <https://tickertape.tdameritrade.com/trading/unbalanced-butterfly-strong-directional-bias-15913>`__
    UnbalancedButterfly,
    UnbalancedCondor,
    UnbalancedIronCondor,
    UnbalancedVerticalRoll,
    /// Mutual fund swap
    MutualFundSwap,
    /// A custom multi-leg order strategy.
    Custom,
}

/// Destinations for when you want to request a specific destination for your order.
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

/// Special instruction for trades.
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpecialInstruction {
    #[default]
    /// Disallow partial order execution.
    /// `More info <https://www.investopedia.com/terms/a/aon.asp>`__.
    AllOrNone,
    /// Do not reduce order size in response to cash dividends.
    /// `More info <https://www.investopedia.com/terms/d/dnr.asp>`__.
    DoNotReduce,
    /// Combination of ``ALL_OR_NONE`` and ``DO_NOT_REDUCE``.
    AllOrNoneDoNotReduce,
}

/// Rules for composite orders.
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStrategyType {
    /// No chaining, only a single order is submitted
    #[default]
    Single,
    Cancel,
    Recall,
    Pair,
    Flatten,
    TwoDaySwap,
    BlastAll,
    /// Execution of one order cancels the other
    Oco,
    /// Execution of one order triggers placement of the other
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

/// Order Status
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionType {
    #[default]
    /// Your order is waiting to be completed.
    Open,
    /// Your order cannot be completed until a certain condition is met.
    OpenContingent,
    /// Your order cancellation request has been submitted.
    CancelPending,
    /// Part of the order has been filled, part is still open.
    OpenPartialFill,
    /// Part of your order was filled. The remaining part was canceled.
    ClosedPartialFill,
    /// Your order was completed in full.
    Fill,
    /// Your order was canceled at your request.
    Canceled,
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
    fn test_de_order_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Order_real.json"
        ));

        let val = serde_json::from_str::<Order>(json);
        println!("{val:?}");
        assert!(val.is_ok());

        // Second real order
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Order_real2.json"
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

    #[test]
    fn test_de_orders_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Orders_real.json"
        ));

        let val = serde_json::from_str::<Vec<Order>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
