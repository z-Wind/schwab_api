/// specifies Parameter for Schwab API
use serde::Deserialize;
use serde::Serialize;
use strum_macros::AsRefStr;

/// Field
///
/// possible root nodes are `quote`, `fundamental`, `extended`, `reference`, `regular`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum QuoteField {
    Quote,
    Fundamental,
    Extended,
    Reference,
    Regular,
    All,
    #[serde(untagged)]
    #[strum(disabled)]
    Extra(String),
}

impl QuoteField {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Extra(s) => s.as_str(),
            _ => self.as_ref(), // 這裡獲取的是 &'static str
        }
    }
}

/// Contract Type
///
/// Available values : `CALL`, `PUT`, `ALL`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContractType {
    Call,
    Put,
    All,
}

/// Option Chain strategy
///
/// Available values : `SINGLE`, `ANALYTICAL`, `COVERED`, `VERTICAL`, `CALENDAR`, `STRANGLE`, `STRADDLE`, `BUTTERFLY`, `CONDOR`, `DIAGONAL`, `COLLAR`, `ROLL`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OptionChainStrategy {
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

/// Expiration month
///
/// Available values : `JAN`, `FEB`, `MAR`, `APR`, `MAY`, `JUN`, `JUL`, `AUG`, `SEP`, `OCT`, `NOV`, `DEC`, `ALL`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
    All,
}

/// Applicable only if its retail token, entitlement of client PP-PayingPro, NP-NonPro and PN-NonPayingPro
///
/// Available values : `PN`, `NP`, `PP`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Entitlement {
    PN,
    NP,
    PP,
}

/// The chart period being requested.
///
/// Available values : `day`, `month`, `year`, `ytd`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PeriodType {
    Day,
    Month,
    Year,
    Ytd,
}

/// The time frequency.
///
/// Available values : `minute`, `daily`, `weekly`, `monthly`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FrequencyType {
    Minute,
    Daily,
    Weekly,
    Monthly,
}

/// Sort by a particular attribute
///
/// Available values : `VOLUME`, `TRADES`, `PERCENT_CHANGE_UP`, `PERCENT_CHANGE_DOWN`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortAttribute {
    Volume,
    Trades,
    PercentChangeUp,
    PercentChangeDown,
}

/// Market
///
/// Available values : `equity`, `option`, `bond`, `future`, `forex`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Market {
    Equity,
    Option,
    Bond,
    Future,
    Forex,
}

/// search by
///
/// Available values : `symbol-search`, `symbol-regex`, `desc-search`, `desc-regex`, `search`, `fundamental`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Projection {
    SymbolSearch,
    SymbolRegex,
    DescSearch,
    DescRegex,
    Search,
    Fundamental,
}

/// Specifies that only orders of this status should be returned.
///
/// Available values : `AWAITING_PARENT_ORDER`, `AWAITING_CONDITION`, `AWAITING_STOP_CONDITION`, `AWAITING_MANUAL_REVIEW`, `ACCEPTED`, `AWAITING_UR_OUT`, `PENDING_ACTIVATION`, `QUEUED`, `WORKING`, `REJECTED`, `PENDING_CANCEL`, `CANCELED`, `PENDING_REPLACE`, `REPLACED`, `FILLED`, `EXPIRED`, `NEW`, `AWAITING_RELEASE_TIME`, `PENDING_ACKNOWLEDGEMENT`, `PENDING_RECALL`, `UNKNOWN`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
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

/// Specifies that only transactions of this status should be returned.
///
/// Available values : `TRADE`, `RECEIVE_AND_DELIVER`, `DIVIDEND_OR_INTEREST`, `ACH_RECEIPT`, `ACH_DISBURSEMENT`, `CASH_RECEIPT`, `CASH_DISBURSEMENT`, `ELECTRONIC_FUND`, `WIRE_OUT`, `WIRE_IN`, `JOURNAL`, `MEMORANDUM`, `MARGIN_CALL`, `MONEY_MARKET`, `SMA_ADJUSTMENT`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Trade,
    ReceiveAndDeliver,
    DividendOrInterest,
    AchReceipt,
    AchDisbursement,
    CashReceipt,
    CashDisbursement,
    ElectronicFund,
    WireOut,
    WireIn,
    Journal,
    Memorandum,
    MarginCall,
    MoneyMarket,
    SmaAdjustment,
}
