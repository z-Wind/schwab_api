use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use super::accounts::AssetType;

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
    pub settlement_date: Option<chrono::DateTime<chrono::Utc>>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferItem {
    pub instrument: TransactionInstrument,
    pub amount: f64,
    pub cost: f64,
    pub price: Option<f64>,
    pub fee_type: Option<TransferItemFeeType>,
    pub position_effect: Option<TransferItemPositionEffect>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
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

// resolve duplicate key "assetType"
impl<'de> Deserialize<'de> for TransactionInstrument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map: serde_json::Map<String, Value> = Deserialize::deserialize(deserializer)?;

        if let Some(Value::String(asset_type)) = map.remove("assetType") {
            match asset_type.as_str() {
                "TRANSACTION_CASH_EQUIVALENT" => {
                    let v = Value::Object(map);
                    serde_json::from_value(v)
                        .map(TransactionInstrument::TransactionCashEquivalent)
                        .map_err(serde::de::Error::custom)
                }
                "COLLECTIVE_INVESTMENT" => {
                    let v = Value::Object(map);
                    serde_json::from_value(v)
                        .map(TransactionInstrument::CollectiveInvestment)
                        .map_err(serde::de::Error::custom)
                }
                "CURRENCY" => {
                    let v = Value::Object(map);
                    serde_json::from_value(v)
                        .map(TransactionInstrument::Currency)
                        .map_err(serde::de::Error::custom)
                }
                "TRANSACTION_EQUITY" => {
                    let v = Value::Object(map);
                    serde_json::from_value(v)
                        .map(TransactionInstrument::TransactionEquity)
                        .map_err(serde::de::Error::custom)
                }
                "TRANSACTION_FIXED_INCOME" => {
                    let v = Value::Object(map);
                    serde_json::from_value(v)
                        .map(TransactionInstrument::TransactionFixedIncome)
                        .map_err(serde::de::Error::custom)
                }
                "FOREX" => {
                    let v = Value::Object(map);
                    serde_json::from_value(v)
                        .map(TransactionInstrument::Forex)
                        .map_err(serde::de::Error::custom)
                }
                "FUTURE" => {
                    let v = Value::Object(map);
                    serde_json::from_value(v)
                        .map(TransactionInstrument::Future)
                        .map_err(serde::de::Error::custom)
                }
                "INDEX" => {
                    let v = Value::Object(map);
                    serde_json::from_value(v)
                        .map(TransactionInstrument::Index)
                        .map_err(serde::de::Error::custom)
                }
                "TRANSACTION_MUTUAL_FUND" => {
                    let v = Value::Object(map);
                    serde_json::from_value(v)
                        .map(TransactionInstrument::TransactionMutualFund)
                        .map_err(serde::de::Error::custom)
                }
                "TRANSACTION_OPTION" => {
                    let v = Value::Object(map);
                    serde_json::from_value(v)
                        .map(TransactionInstrument::TransactionOption)
                        .map_err(serde::de::Error::custom)
                }
                "PRODUCT" => {
                    let v = Value::Object(map);
                    serde_json::from_value(v)
                        .map(TransactionInstrument::Product)
                        .map_err(serde::de::Error::custom)
                }
                _ => Err(serde::de::Error::unknown_variant(
                    &asset_type,
                    &[
                        "TRANSACTION_CASH_EQUIVALENT",
                        "COLLECTIVE_INVESTMENT",
                        "CURRENCY",
                        "TRANSACTION_EQUITY",
                        "TRANSACTION_FIXED_INCOME",
                        "FOREX",
                        "FUTURE",
                        "INDEX",
                        "TRANSACTION_MUTUAL_FUND",
                        "TRANSACTION_OPTION",
                        "PRODUCT",
                    ],
                )),
            }
        } else {
            Err(serde::de::Error::missing_field("assetType"))
        }
    }
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
    pub factor: f64,
    pub multiplier: f64,
    pub variable_rate: f64,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    /// default: false
    pub active_contract: bool,
    #[serde(rename = "type")]
    pub type_field: IndexType,

    #[serde(flatten)]
    pub transaction_instrument: Box<TransactionInstrument>,
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
    pub put_call: TransactionOptionPullCall,
    pub strike_price: f64,
    #[serde(rename = "type")]
    pub type_field: TransactionOptionType,
    pub underlying_symbol: String,
    pub underlying_cusip: String,
    pub deliverable: Box<TransactionInstrument>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAPIOptionDeliverable {
    pub root_symbol: String,
    pub strike_percent: i64,
    pub deliverable_number: i64,
    pub deliverable_units: f64,
    pub deliverable: TransactionInstrument,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionBaseInstrument {
    pub cusip: Option<String>,
    pub symbol: String,
    pub description: Option<String>,
    pub instrument_id: i64,
    pub net_change: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionCashEquivalentType {
    SweepVehicle,
    Savings,
    MoneyMarketFund,
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
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ForexType {
    Standard,
    Nbbo,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FutureType {
    Standard,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndexType {
    BroadBased,
    NarrowBased,
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
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionOptionPullCall {
    Put,
    Call,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductType {
    Tbd,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionOptionType {
    Vanilla,
    Binary,
    Barrier,
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
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionActivityType {
    ActivityCorrection,
    Execution,
    OrderAction,
    Transfer,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserDetailsType {
    AdvisorUser,
    BrokerUser,
    ClientUser,
    SystemUser,
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
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferItemPositionEffect {
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
            "/tests/model/Trader/Transactions.json"
        ));

        let val = serde_json::from_str::<Vec<Transaction>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_de_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Transactions_real.json"
        ));

        let val = serde_json::from_str::<Vec<Transaction>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_de_real2() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Transaction_real.json"
        ));

        let val = serde_json::from_str::<Transaction>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
