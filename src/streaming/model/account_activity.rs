use serde::{Deserialize, Serialize};

use crate::Error;

// ─────────────────────────────────────────────────────────────
// Shared primitives
// ─────────────────────────────────────────────────────────────

/// Schwab's proprietary fixed-point number.
/// `lo` is the raw integer; `sign_scale` encodes sign and decimal position.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchwabDecimal {
    #[serde(default)]
    pub lo: Option<String>,
    #[serde(rename = "signScale", default)]
    pub sign_scale: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeValue {
    #[serde(rename = "DateTimeString")]
    pub date_time_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Security {
    #[serde(rename = "SchwabSecurityID")]
    pub schwab_security_id: String,
    #[serde(rename = "Symbol")]
    pub symbol: String,
    #[serde(rename = "UnderlyingSymbol", default)]
    pub underlying_symbol: Option<String>,
    #[serde(rename = "PrimaryExchangeCode", default)]
    pub primary_exchange_code: Option<String>,
    #[serde(rename = "MajorAssetType", default)]
    pub major_asset_type: Option<String>,
    #[serde(rename = "ShortDescriptionText", default)]
    pub short_description: Option<String>,
    #[serde(rename = "CUSIP", default)]
    pub cusip: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLeg {
    #[serde(rename = "LegID")]
    pub leg_id: String,
    #[serde(rename = "BuySellCode")]
    pub buy_sell_code: String,
    #[serde(rename = "Quantity")]
    pub quantity: SchwabDecimal,
    #[serde(rename = "LeavesQuantity", default)]
    pub leaves_quantity: Option<SchwabDecimal>,
    #[serde(rename = "Security")]
    pub security: Security,
}

// ─────────────────────────────────────────────────────────────
// OrderCreated
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedEquityOrder {
    #[serde(rename = "AssetType", default)]
    pub asset_type: Option<String>,
    #[serde(rename = "TimeInForce", default)]
    pub time_in_force: Option<String>,
    #[serde(rename = "OrderTypeCode", default)]
    pub order_type_code: Option<String>,
    #[serde(rename = "OrderLegs", default)]
    pub order_legs: Vec<OrderLeg>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedInner {
    #[serde(rename = "SchwabOrderID")]
    pub schwab_order_id: String,
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    #[serde(rename = "Order")]
    pub order: OrderCreatedEquityOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedEvent {
    #[serde(rename = "Order")]
    pub order: OrderCreatedInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedBaseEvent {
    #[serde(rename = "OrderCreatedEventEquityOrder")]
    pub order_created: OrderCreatedEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreated {
    #[serde(rename = "SchwabOrderID")]
    pub schwab_order_id: String,
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    #[serde(rename = "BaseEvent")]
    pub base_event: OrderCreatedBaseEvent,
}

impl OrderCreated {
    pub fn legs(&self) -> &[OrderLeg] {
        &self.base_event.order_created.order.order.order_legs
    }
}

// ─────────────────────────────────────────────────────────────
// OrderAccepted
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteEntry {
    #[serde(rename = "Ask", default)]
    pub ask: Option<SchwabDecimal>,
    #[serde(rename = "AskSize", default)]
    pub ask_size: Option<SchwabDecimal>,
    #[serde(rename = "Bid", default)]
    pub bid: Option<SchwabDecimal>,
    #[serde(rename = "BidSize", default)]
    pub bid_size: Option<SchwabDecimal>,
    #[serde(rename = "Mid", default)]
    pub mid: Option<SchwabDecimal>,
    #[serde(rename = "Symbol")]
    pub symbol: String,
    #[serde(rename = "QuoteTypeCode", default)]
    pub quote_type_code: Option<String>,
    #[serde(rename = "QuoteTimestamp", default)]
    pub quote_timestamp: Option<DateTimeValue>,
    #[serde(rename = "SchwabOrderID", default)]
    pub schwab_order_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAcceptedEvent {
    #[serde(rename = "CreatedTimeStamp")]
    pub created_timestamp: DateTimeValue,
    #[serde(rename = "ExpiryTimeStamp", default)]
    pub expiry_timestamp: Option<DateTimeValue>,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "TradingSessionCodeOnOrderEntry", default)]
    pub trading_session_code: Option<String>,
    #[serde(rename = "QuoteOnOrderEntry", default)]
    pub quote_on_order_entry: Vec<QuoteEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAcceptedBaseEvent {
    #[serde(rename = "OrderAcceptedEvent")]
    pub order_accepted: OrderAcceptedEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAccepted {
    #[serde(rename = "SchwabOrderID")]
    pub schwab_order_id: String,
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    #[serde(rename = "BaseEvent")]
    pub base_event: OrderAcceptedBaseEvent,
}

// ─────────────────────────────────────────────────────────────
// CancelAccepted
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegCancelInfo {
    #[serde(rename = "LegID")]
    pub leg_id: String,
    #[serde(rename = "IntendedOrderQuantity", default)]
    pub intended_quantity: Option<SchwabDecimal>,
    #[serde(rename = "RequestedAmount", default)]
    pub requested_amount: Option<SchwabDecimal>,
    #[serde(rename = "LegStatus", default)]
    pub leg_status: Option<String>,
    #[serde(rename = "LegSubStatus", default)]
    pub leg_sub_status: Option<String>,
    #[serde(rename = "CancelAcceptedTime", default)]
    pub cancel_accepted_time: Option<DateTimeValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelAcceptedEvent {
    #[serde(rename = "LifecycleSchwabOrderID")]
    pub lifecycle_schwab_order_id: String,
    #[serde(rename = "CancelTimeStamp")]
    pub cancel_timestamp: DateTimeValue,
    #[serde(rename = "LegCancelRequestInfoList", default)]
    pub leg_cancel_request_info_list: Vec<LegCancelInfo>,
    #[serde(rename = "CancelRequestType", default)]
    pub cancel_request_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelAcceptedBaseEvent {
    #[serde(rename = "CancelAcceptedEvent")]
    pub cancel_accepted: CancelAcceptedEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelAccepted {
    #[serde(rename = "SchwabOrderID")]
    pub schwab_order_id: String,
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    #[serde(rename = "BaseEvent")]
    pub base_event: CancelAcceptedBaseEvent,
}

// ─────────────────────────────────────────────────────────────
// ExecutionCreated
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionInfo {
    #[serde(rename = "ExecutionSequenceNumber")]
    pub sequence_number: i64,
    #[serde(rename = "ExecutionId")]
    pub execution_id: String,
    #[serde(rename = "ExecutionQuantity", default)]
    pub quantity: Option<SchwabDecimal>,
    #[serde(rename = "ExecutionTimeStamp")]
    pub timestamp: DateTimeValue,
    #[serde(rename = "ExecutionTransType", default)]
    pub trans_type: Option<String>,
    #[serde(rename = "ExecutionCapacityCode", default)]
    pub capacity_code: Option<String>,
    #[serde(rename = "RouteName", default)]
    pub route_name: Option<String>,
    #[serde(rename = "CancelType", default)]
    pub cancel_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCreatedEvent {
    #[serde(rename = "LegId")]
    pub leg_id: String,
    #[serde(rename = "ExecutionInfo")]
    pub execution_info: ExecutionInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCreatedBaseEvent {
    #[serde(rename = "ExecutionCreatedEventExecutionInfo")]
    pub execution_created: ExecutionCreatedEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCreated {
    #[serde(rename = "SchwabOrderID")]
    pub schwab_order_id: String,
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    #[serde(rename = "BaseEvent")]
    pub base_event: ExecutionCreatedBaseEvent,
}

// ─────────────────────────────────────────────────────────────
// OrderMonitorCreated
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMonitorCreatedEvent {
    #[serde(rename = "LegId")]
    pub leg_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMonitorCreatedBaseEvent {
    #[serde(rename = "OrderMonitorCreatedEvent")]
    pub order_monitor_created: OrderMonitorCreatedEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMonitorCreated {
    #[serde(rename = "SchwabOrderID")]
    pub schwab_order_id: String,
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    #[serde(rename = "BaseEvent")]
    pub base_event: OrderMonitorCreatedBaseEvent,
}

// ─────────────────────────────────────────────────────────────
// OrderMonitorCompleted
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMonitorCompletedEvent {
    #[serde(rename = "LegId")]
    pub leg_id: String,
    #[serde(rename = "CreditRiskCheckIndicator", default)]
    pub credit_risk_check_indicator: Option<bool>,
    #[serde(rename = "PlanSubmitDate", default)]
    pub plan_submit_date: Option<DateTimeValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMonitorCompletedBaseEvent {
    #[serde(rename = "OrderMonitorCompletedEvent")]
    pub order_monitor_completed: OrderMonitorCompletedEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMonitorCompleted {
    #[serde(rename = "SchwabOrderID")]
    pub schwab_order_id: String,
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    #[serde(rename = "BaseEvent")]
    pub base_event: OrderMonitorCompletedBaseEvent,
}

// ─────────────────────────────────────────────────────────────
// ChangeCreated
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeCreatedEvent {
    #[serde(rename = "Order")]
    pub order: OrderCreatedInner, // same shape as OrderCreated
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeCreatedBaseEvent {
    #[serde(rename = "ChangeCreatedEventEquityOrder")]
    pub change_created: ChangeCreatedEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeCreated {
    #[serde(rename = "SchwabOrderID")]
    pub schwab_order_id: String,
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    #[serde(rename = "ParentSchwabOrderID", default)]
    pub parent_schwab_order_id: Option<String>,
    #[serde(rename = "LifecycleSchwabOrderID", default)]
    pub lifecycle_schwab_order_id: Option<String>,
    #[serde(rename = "BaseEvent")]
    pub base_event: ChangeCreatedBaseEvent,
}

// ─────────────────────────────────────────────────────────────
// ChangeAccepted
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegInfoUpdate {
    #[serde(rename = "LegId")]
    pub leg_id: String,
    #[serde(rename = "AccountingRuleCode", default)]
    pub accounting_rule_code: Option<String>,
    #[serde(rename = "IntendedOrderQuantity", default)]
    pub intended_order_quantity: Option<SchwabDecimal>,
    #[serde(rename = "PreviousLegId", default)]
    pub previous_leg_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeAcceptedEvent {
    #[serde(rename = "CreatedTimeStamp")]
    pub created_timestamp: DateTimeValue,
    #[serde(rename = "ExpiryTimeStamp", default)]
    pub expiry_timestamp: Option<DateTimeValue>,
    #[serde(rename = "Status", default)]
    pub status: Option<String>,
    #[serde(rename = "LegStatus", default)]
    pub leg_status: Option<String>,
    #[serde(rename = "TradingSessionCodeOnOrderEntry", default)]
    pub trading_session_code: Option<String>,
    #[serde(rename = "QuoteOnOrderEntry", default)]
    pub quote_on_order_entry: Vec<QuoteEntry>,
    #[serde(rename = "LegInfoUpdate", default)]
    pub leg_info_update: Vec<LegInfoUpdate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeAcceptedBaseEvent {
    #[serde(rename = "ChangeAcceptedEvent")]
    pub change_accepted: ChangeAcceptedEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeAccepted {
    #[serde(rename = "SchwabOrderID")]
    pub schwab_order_id: String,
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    #[serde(rename = "BaseEvent")]
    pub base_event: ChangeAcceptedBaseEvent,
}

// ─────────────────────────────────────────────────────────────
// OrderUROutCompleted
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderUROutCompletedEvent {
    #[serde(rename = "LegId")]
    pub leg_id: String,
    #[serde(rename = "ExecutionId")]
    pub execution_id: String,
    #[serde(rename = "LeavesQuantity", default)]
    pub leaves_quantity: Option<SchwabDecimal>,
    #[serde(rename = "CancelQuantity", default)]
    pub cancel_quantity: Option<SchwabDecimal>,
    #[serde(rename = "LegStatus", default)]
    pub leg_status: Option<String>,
    #[serde(rename = "LegSubStatus", default)]
    pub leg_sub_status: Option<String>,
    #[serde(rename = "OutCancelType", default)]
    pub out_cancel_type: Option<String>,
    #[serde(rename = "ExecutionTimeStamp")]
    pub execution_timestamp: DateTimeValue,
    #[serde(rename = "RouteName", default)]
    pub route_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderUROutCompletedBaseEvent {
    #[serde(rename = "OrderUROutCompletedEvent")]
    pub ur_out_completed: OrderUROutCompletedEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderUROutCompleted {
    #[serde(rename = "SchwabOrderID")]
    pub schwab_order_id: String,
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    #[serde(rename = "BaseEvent")]
    pub base_event: OrderUROutCompletedBaseEvent,
}

// ─────────────────────────────────────────────────────────────
// AccountActivity
// ─────────────────────────────────────────────────────────────

/// A parsed account activity notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountActivity {
    /// The subscription key (`SchwabClientCustomerId` hash).
    pub key: String,
    /// The account number this activity belongs to.
    pub account_number: String,
    /// The typed event payload.
    pub event: AccountActivityEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountActivityEvent {
    /// Subscription confirmed — no payload.
    Subscribed,
    OrderCreated(OrderCreated),
    OrderAccepted(OrderAccepted),
    CancelAccepted(CancelAccepted),
    ChangeCreated(ChangeCreated),
    ChangeAccepted(ChangeAccepted),
    OrderMonitorCompleted(OrderMonitorCompleted),
    OrderMonitorCreated(OrderMonitorCreated),
    ExecutionCreated(ExecutionCreated),
    OrderUROutCompleted(OrderUROutCompleted),
}

impl AccountActivityEvent {
    pub(super) fn from_raw(message_type: &str, data: String) -> Result<Self, Error> {
        Ok(match message_type {
            "SUBSCRIBED" => Self::Subscribed,
            "OrderCreated" => Self::OrderCreated(serde_json::from_str(&data)?),
            "OrderAccepted" => Self::OrderAccepted(serde_json::from_str(&data)?),
            "CancelAccepted" => Self::CancelAccepted(serde_json::from_str(&data)?),
            "ChangeCreated" => Self::ChangeCreated(serde_json::from_str(&data)?),
            "ChangeAccepted" => Self::ChangeAccepted(serde_json::from_str(&data)?),
            "ExecutionCreated" => Self::ExecutionCreated(serde_json::from_str(&data)?),
            "OrderMonitorCreated" => Self::OrderMonitorCreated(serde_json::from_str(&data)?),
            "OrderMonitorCompleted" => Self::OrderMonitorCompleted(serde_json::from_str(&data)?),
            "OrderUROutCompleted" => Self::OrderUROutCompleted(serde_json::from_str(&data)?),
            _ => {
                return Err(Error::Streaming(format!(
                    "Unknown account activity type: {message_type}"
                )));
            }
        })
    }
}
