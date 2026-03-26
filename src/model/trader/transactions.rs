use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use serde_json::{Map, Value};

use super::accounts::AssetType;

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub activity_id: i64,
    pub time: String,
    pub user: Option<UserDetails>,
    pub description: Option<String>,
    pub account_number: String,
    #[serde(rename = "type")]
    pub type_field: TransactionType,
    pub status: TransactionStatus,
    pub sub_account: TransactionSubAccount,
    pub trade_date: chrono::DateTime<chrono::Utc>,
    pub settlement_date: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub position_id: Option<i64>,
    pub order_id: Option<i64>,
    pub net_amount: f64,
    pub activity_type: Option<TransactionActivityType>,
    /// xml: `OrderedMap` { "name": "transferItems", "wrapped": true }
    pub transfer_items: Vec<TransferItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
    pub cd_domain_id: String,
    pub login: String,
    #[serde(rename = "type")]
    pub type_field: UserDetailsType,
    pub user_id: i64,
    pub system_user_name: String,
    pub first_name: String,
    pub last_name: String,
    pub broker_rep_code: String,
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferItem {
    pub instrument: DuplicatedKey<TransactionInstrument>,
    pub amount: f64,
    pub cost: f64,
    pub price: Option<f64>,
    pub fee_type: Option<TransferItemFeeType>,
    pub position_effect: Option<TransferItemPositionEffect>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(transparent)]
pub struct DuplicatedKey<T: DeserializeOwned>(T);

impl<'de, T: DeserializeOwned> Deserialize<'de> for DuplicatedKey<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = Map::<String, Value>::deserialize(deserializer)?;

        T::deserialize(Value::Object(map))
            .map(DuplicatedKey)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "assetType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionInstrument {
    TransactionCashEquivalent(TransactionCashEquivalent),
    CollectiveInvestment(CollectiveInvestment),
    Currency(Currency),
    TransactionEquity(TransactionEquity),
    TransactionFixedIncome(TransactionFixedIncome),
    Forex(Forex),
    Future(Future),
    Index(Index),
    TransactionMutualFund(TransactionMutualFund),
    TransactionOption(TransactionOption),
    Product(Product),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionCashEquivalent {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    #[serde(rename = "type")]
    pub type_field: TransactionCashEquivalentType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectiveInvestment {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    #[serde(rename = "type")]
    pub type_field: CollectiveInvestmentType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEquity {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    #[serde(rename = "type")]
    pub type_field: TransactionEquityType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFixedIncome {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    #[serde(rename = "type")]
    pub type_field: TransactionFixedIncomeType,
    pub maturity_date: chrono::DateTime<chrono::Utc>,
    pub factor: Option<f64>,
    pub multiplier: Option<f64>,
    pub variable_rate: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forex {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    #[serde(rename = "type")]
    pub type_field: ForexType,
    pub base_currency: Currency,
    pub counter_currency: Currency,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Future {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    /// default: false
    #[serde(default)]
    pub active_contract: bool,
    #[serde(rename = "type")]
    pub type_field: FutureType,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
    pub last_trading_date: chrono::DateTime<chrono::Utc>,
    pub first_notice_date: chrono::DateTime<chrono::Utc>,
    pub multiplier: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    /// default: false
    #[serde(default)]
    pub active_contract: bool,
    #[serde(rename = "type")]
    pub type_field: IndexType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMutualFund {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    pub fund_family_name: String,
    pub fund_family_symbol: String,
    pub fund_group: String,
    #[serde(rename = "type")]
    pub type_field: TransactionMutualFundType,
    pub exchange_cutoff_time: chrono::DateTime<chrono::Utc>,
    pub purchase_cutoff_time: chrono::DateTime<chrono::Utc>,
    pub redemption_cutoff_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionOption {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    pub expiration_date: chrono::DateTime<chrono::Utc>,
    /// xml: `OrderedMap` { "name": "optionDeliverables", "wrapped": true }
    pub option_deliverables: Vec<TransactionAPIOptionDeliverable>,
    pub option_premium_multiplier: i64,
    pub put_call: TransactionOptionPutCall,
    pub strike_price: f64,
    #[serde(rename = "type")]
    pub type_field: TransactionOptionType,
    pub underlying_symbol: String,
    pub underlying_cusip: String,
    pub deliverable: Box<DuplicatedKey<TransactionInstrument>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAPIOptionDeliverable {
    pub root_symbol: String,
    pub strike_percent: i64,
    pub deliverable_number: i64,
    pub deliverable_units: f64,
    pub deliverable: DuplicatedKey<TransactionInstrument>,
    pub asset_type: AssetType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    #[serde(rename = "type")]
    pub type_field: ProductType,
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionBaseInstrument {
    pub cusip: Option<String>,
    pub symbol: String,
    pub description: Option<String>,
    pub instrument_id: Option<i64>,
    pub net_change: Option<f64>,

    // Fields not explicitly defined in the official schema
    pub status: Option<String>,
    pub closing_price: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionCashEquivalentType {
    SweepVehicle,
    Savings,
    MoneyMarketFund,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CollectiveInvestmentType {
    UnitInvestmentTrust,
    ExchangeTradedFund,
    ClosedEndFund,
    Index,
    Units,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionEquityType {
    CommonStock,
    PreferredStock,
    DepositoryReceipt,
    PreferredDepositoryReceipt,
    RestrictedStock,
    ComponentUnit,
    Right,
    Warrant,
    ConvertiblePreferredStock,
    ConvertibleStock,
    LimitedPartnership,
    WhenIssued,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionFixedIncomeType {
    BondUnit,
    CertificateOfDeposit,
    ConvertibleBond,
    CollateralizedMortgageObligation,
    CorporateBond,
    GovernmentMortgage,
    GnmaBonds,
    MunicipalAssessmentDistrict,
    MunicipalBond,
    OtherGovernment,
    ShortTermPaper,
    UsTreasuryBond,
    UsTreasuryBill,
    UsTreasuryNote,
    UsTreasuryZeroCoupon,
    AgencyBond,
    WhenAsAndIfIssuedBond,
    AssetBackedSecurity,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ForexType {
    Standard,
    Nbbo,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FutureType {
    Standard,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndexType {
    BroadBased,
    NarrowBased,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionMutualFundType {
    NotApplicable,
    OpenEndNonTaxable,
    OpenEndTaxable,
    NoLoadNonTaxable,
    NoLoadTaxable,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionOptionPutCall {
    Put,
    Call,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductType {
    Tbd,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionOptionType {
    Vanilla,
    Binary,
    Barrier,
    #[serde(other)]
    Unknown,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Valid,
    Invalid,
    Pending,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionSubAccount {
    Cash,
    Margin,
    Short,
    Div,
    Income,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for TransactionSubAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_string(self).unwrap_or_else(|_| "UNKNOWN".to_string());
        write!(f, "{}", s.replace('"', ""))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionActivityType {
    ActivityCorrection,
    Execution,
    OrderAction,
    Transfer,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserDetailsType {
    AdvisorUser,
    BrokerUser,
    ClientUser,
    SystemUser,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferItemFeeType {
    Commission,
    SecFee,
    StrFee,
    RFee,
    CdscFee,
    OptRegFee,
    AdditionalFee,
    MiscellaneousFee,
    FuturesExchangeFee,
    LowProceedsCommission,
    BaseCharge,
    GeneralCharge,
    GstFee,
    TafFee,
    IndexOptionFee,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferItemPositionEffect {
    Opening,
    Closing,
    Automatic,
    #[serde(other)]
    Unknown,
}

#[cfg(test)]
mod tests {
    use assert_json_diff::{CompareMode, Config, NumericMode, assert_json_matches};
    use regex::Regex;
    use test_log::test;

    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Transactions.json"
        ));

        let val = serde_json::from_str::<Vec<Transaction>>(json);
        tracing::debug!(?val);

        assert!(val.is_ok(), "Failed to deserialize: {:?}", val.err());
    }

    #[test]
    fn test_serde_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Transactions_real.json"
        ));

        // 1. Create a more robust regex to handle multiple ISO 8601 variations:
        // - (\.000)? : Optional milliseconds
        // - (\+00:00|\+0000) : Timezone as +00:00 or +0000
        let re = Regex::new(r"(\.000)?(\+00:00|\+0000)").unwrap();

        // 2. Normalize all variations to "Z" to match Rust's default output
        let json = re.replace_all(json, "Z");

        let json: serde_json::Value = serde_json::from_str(&json).unwrap();

        let val = serde_json::from_value::<Vec<Transaction>>(json.clone()).unwrap();
        tracing::debug!(?val);

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
            "/tests/model/Trader/Transaction_real.json"
        ));

        // 1. Create a more robust regex to handle multiple ISO 8601 variations:
        // - (\.000)? : Optional milliseconds
        // - (\+00:00|\+0000) : Timezone as +00:00 or +0000
        let re = Regex::new(r"(\.000)?(\+00:00|\+0000)").unwrap();

        // 2. Normalize all variations to "Z" to match Rust's default output
        let json = re.replace_all(json, "Z");

        let json: serde_json::Value = serde_json::from_str(&json).unwrap();

        let val = serde_json::from_value::<Transaction>(json.clone()).unwrap();
        tracing::debug!(?val);

        assert_json_matches!(
            val,
            json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat)
        );
    }
}
