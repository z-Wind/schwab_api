use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Command {
    // ── ADMIN ────────────────────────────────────────────────
    #[serde(rename = "LOGIN")]
    Login,
    #[serde(rename = "LOGOUT")]
    Logout,
    #[serde(rename = "QOS")]
    Qos,

    // ── Subscription commands ────────────────────────────────
    #[serde(rename = "SUBS")]
    Subscriptions,
    #[serde(rename = "ADD")]
    Add,
    #[serde(rename = "UNSUBS")]
    Unsubs,
    #[serde(rename = "VIEW")]
    View,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "STREAM")]
    Stream,

    // ── ACCT_ACTIVITY message types ──────────────────────────
    #[serde(rename = "SUBSCRIBED")]
    Subscribed,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "BrokenTrade")]
    BrokenTrade,
    #[serde(rename = "ManualExecution")]
    ManualExecution,
    #[serde(rename = "OrderActivation")]
    OrderActivation,
    #[serde(rename = "OrderCancelReplaceRequest")]
    OrderCancelReplaceRequest,
    #[serde(rename = "OrderCancelRequest")]
    OrderCancelRequest,
    #[serde(rename = "OrderEntryRequest")]
    OrderEntryRequest,
    #[serde(rename = "OrderFill")]
    OrderFill,
    #[serde(rename = "OrderPartialFill")]
    OrderPartialFill,
    #[serde(rename = "OrderRejection")]
    OrderRejection,
    #[serde(rename = "TooLateToCancel")]
    TooLateToCancel,
    #[serde(rename = "UROUT")]
    Urout,
}
