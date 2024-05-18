use serde::Deserialize;
use serde::Serialize;

use super::accounts::AssetType;

pub type Root = Vec<Transaction>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub settlement_date: Option<chrono::DateTime<chrono::Utc>>,
    pub position_id: Option<i64>,
    pub order_id: Option<i64>,
    pub net_amount: f64,
    pub activity_type: Option<TransactionActivityType>,
    /// xml: `OrderedMap` { "name": "transferItems", "wrapped": true }
    pub transfer_items: Vec<TransferItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferItem {
    pub instrument: TransactionInstrument,
    pub amount: f64,
    pub cost: f64,
    pub price: Option<f64>,
    pub fee_type: Option<TransferItemFeeType>,
    pub position_effect: Option<TransferItemPositionEffect>,
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

impl Default for TransactionInstrument {
    fn default() -> Self {
        Self::TransactionEquity(TransactionEquity::default())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionCashEquivalent {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    #[serde(rename = "type")]
    pub type_field: TransactionCashEquivalentType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectiveInvestment {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    #[serde(rename = "type")]
    pub type_field: CollectiveInvestmentType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEquity {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    #[serde(rename = "type")]
    pub type_field: TransactionEquityType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFixedIncome {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    #[serde(rename = "type")]
    pub type_field: TransactionFixedIncomeType,
    pub maturity_date: chrono::DateTime<chrono::Utc>,
    pub factor: f64,
    pub multiplier: f64,
    pub variable_rate: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forex {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    #[serde(rename = "type")]
    pub type_field: ForexType,
    pub base_currency: Currency,
    pub counter_currency: Currency,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Future {
    /// default: false
    pub active_contract: bool,
    #[serde(rename = "type")]
    pub type_field: FutureType,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
    pub last_trading_date: chrono::DateTime<chrono::Utc>,
    pub first_notice_date: chrono::DateTime<chrono::Utc>,
    pub multiplier: f64,

    #[serde(flatten)]
    pub transaction_instrument: Box<TransactionInstrument>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    /// default: false
    pub active_contract: bool,
    #[serde(rename = "type")]
    pub type_field: IndexType,

    #[serde(flatten)]
    pub transaction_instrument: Box<TransactionInstrument>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionOption {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    pub expiration_date: chrono::DateTime<chrono::Utc>,
    /// xml: `OrderedMap` { "name": "optionDeliverables", "wrapped": true }
    pub option_deliverables: Vec<TransactionAPIOptionDeliverable>,
    pub option_premium_multiplier: i64,
    pub put_call: TransactionOptionPullCall,
    pub strike_price: f64,
    #[serde(rename = "type")]
    pub type_field: TransactionOptionType,
    pub underlying_symbol: String,
    pub underlying_cusip: String,
    pub deliverable: Box<TransactionInstrument>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAPIOptionDeliverable {
    pub root_symbol: String,
    pub strike_percent: i64,
    pub deliverable_number: i64,
    pub deliverable_units: f64,
    pub deliverable: TransactionInstrument,
    pub asset_type: AssetType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    #[serde(flatten)]
    pub transaction_base_instrument: TransactionBaseInstrument,

    #[serde(rename = "type")]
    pub type_field: ProductType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionBaseInstrument {
    pub cusip: Option<String>,
    pub symbol: String,
    pub description: Option<String>,
    pub instrument_id: i64,
    pub net_change: Option<f64>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionAssetType {
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
pub enum TransactionCashEquivalentType {
    #[default]
    SweepVehicle,
    Savings,
    MoneyMarketFund,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CollectiveInvestmentType {
    #[default]
    UnitInvestmentTrust,
    ExchangeTradedFund,
    ClosedEndFund,
    Index,
    Units,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionEquityType {
    #[default]
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
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionFixedIncomeType {
    #[default]
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
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ForexType {
    #[default]
    Standard,
    Nbbo,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FutureType {
    #[default]
    Standard,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndexType {
    #[default]
    BroadBased,
    NarrowBased,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionMutualFundType {
    #[default]
    NotApplicable,
    OpenEndNonTaxable,
    OpenEndTaxable,
    NoLoadNonTaxable,
    NoLoadTaxable,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionOptionPullCall {
    #[default]
    Put,
    Call,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductType {
    #[default]
    Tbd,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionOptionType {
    #[default]
    Vanilla,
    Binary,
    Barrier,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    #[default]
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    #[default]
    Valid,
    Invalid,
    Pending,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionSubAccount {
    #[default]
    Cash,
    Margin,
    Short,
    Div,
    Income,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionActivityType {
    #[default]
    ActivityCorrection,
    Execution,
    OrderAction,
    Transfer,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserDetailsType {
    #[default]
    AdvisorUser,
    BrokerUser,
    ClientUser,
    SystemUser,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferItemFeeType {
    #[default]
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
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferItemPositionEffect {
    #[default]
    Opening,
    Closing,
    Automatic,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Transactions_real.json"
        ));

        let val = serde_json::from_str::<Vec<Transaction>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_de2() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Transactions_real.json"
        ));

        let val = serde_json::from_str::<Vec<Transaction>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
