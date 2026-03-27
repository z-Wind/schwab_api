use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::Number;

pub type Accounts = Vec<Account>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub securities_account: SecuritiesAccount,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
pub enum SecuritiesAccount {
    Margin(Box<MarginAccount>),
    Cash(Box<CashAccount>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesAccountBase {
    pub account_number: String,
    pub round_trips: i64,
    /// default: false
    #[serde(default)]
    pub is_day_trader: bool,
    /// default: false
    #[serde(default)]
    pub is_closing_only_restricted: bool,
    /// default: false
    #[serde(default)]
    pub pfcb_flag: bool,
    pub positions: Option<Vec<Position>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginAccount {
    #[serde(flatten)]
    pub securities_account_base: SecuritiesAccountBase,
    pub initial_balances: Option<MarginInitialBalance>,
    pub current_balances: Option<MarginBalance>,
    pub projected_balances: Option<MarginBalance>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginInitialBalance {
    pub accrued_interest: Number,
    pub available_funds_non_marginable_trade: Number,
    pub bond_value: Number,
    pub buying_power: Number,
    pub cash_balance: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_available_for_trading: Option<Number>,
    pub cash_receipts: Number,
    pub day_trading_buying_power: Number,
    pub day_trading_buying_power_call: Number,
    pub day_trading_equity_call: Number,
    pub equity: Number,
    pub equity_percentage: Number,
    pub liquidation_value: Number,
    pub long_margin_value: Number,
    pub long_option_market_value: Number,
    pub long_stock_value: Number,
    pub maintenance_call: Number,
    pub maintenance_requirement: Number,
    pub margin: Number,
    pub margin_equity: Number,
    pub money_market_fund: Number,
    pub mutual_fund_value: Number,
    pub reg_t_call: Number,
    pub short_margin_value: Number,
    pub short_option_market_value: Number,
    pub short_stock_value: Number,
    pub total_cash: Number,
    pub is_in_call: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsettled_cash: Option<Number>,
    pub pending_deposits: Number,
    pub margin_balance: Number,
    pub short_balance: Number,
    pub account_value: Number,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginBalance {
    pub available_funds: Option<Number>,
    pub available_funds_non_marginable_trade: Option<Number>,
    pub buying_power: Option<Number>,
    pub buying_power_non_marginable_trade: Option<Number>,
    pub day_trading_buying_power: Option<Number>,
    pub day_trading_buying_power_call: Option<Number>,
    pub equity: Option<Number>,
    pub equity_percentage: Option<Number>,
    pub long_margin_value: Option<Number>,
    pub maintenance_call: Option<Number>,
    pub maintenance_requirement: Option<Number>,
    pub margin_balance: Option<Number>,
    pub reg_t_call: Option<Number>,
    pub short_balance: Option<Number>,
    pub short_margin_value: Option<Number>,
    pub sma: Option<Number>,
    pub is_in_call: Option<bool>,
    pub stock_buying_power: Option<Number>,
    pub option_buying_power: Option<Number>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashAccount {
    #[serde(flatten)]
    pub securities_account_base: SecuritiesAccountBase,

    pub initial_balances: Option<CashInitialBalance>,
    pub current_balances: Option<CashBalance>,
    pub projected_balances: Option<CashBalance>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashInitialBalance {
    pub accrued_interest: Number,
    pub cash_available_for_trading: Number,
    pub cash_available_for_withdrawal: Number,
    pub cash_balance: Number,
    pub bond_value: Number,
    pub cash_receipts: Number,
    pub liquidation_value: Number,
    pub long_option_market_value: Number,
    pub long_stock_value: Number,
    pub money_market_fund: Number,
    pub mutual_fund_value: Number,
    pub short_option_market_value: Number,
    pub short_stock_value: Number,
    pub is_in_call: bool,
    pub unsettled_cash: Number,
    pub cash_debit_call_value: Number,
    pub pending_deposits: Number,
    pub account_value: Number,
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashBalance {
    pub cash_available_for_trading: Number,
    pub cash_available_for_withdrawal: Number,
    pub cash_call: Option<Number>,
    pub long_non_marginable_market_value: Option<Number>,
    pub total_cash: Option<Number>,
    pub cash_debit_call_value: Option<Number>,
    pub unsettled_cash: Option<Number>,

    // Fields not explicitly defined in the official schema
    // ===================================================
    pub accrued_interest: Option<Number>,
    pub cash_balance: Option<Number>,
    pub cash_receipts: Option<Number>,
    pub long_option_market_value: Option<Number>,
    pub liquidation_value: Option<Number>,
    pub long_market_value: Option<Number>,
    pub money_market_fund: Option<Number>,
    pub savings: Option<Number>,
    pub short_market_value: Option<Number>,
    pub pending_deposits: Option<Number>,
    pub mutual_fund_value: Option<Number>,
    pub bond_value: Option<Number>,
    pub short_option_market_value: Option<Number>,
    // ===================================================
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub short_quantity: Number,
    pub average_price: Number,
    pub current_day_profit_loss: Number,
    pub current_day_profit_loss_percentage: Number,
    pub long_quantity: Number,
    pub settled_long_quantity: Number,
    pub settled_short_quantity: Number,
    pub aged_quantity: Number,
    pub instrument: AccountsInstrument,
    pub market_value: Number,
    pub maintenance_requirement: Number,
    pub average_long_price: Number,
    pub average_short_price: Number,
    pub tax_lot_average_long_price: Number,
    pub tax_lot_average_short_price: Number,
    pub long_open_profit_loss: Number,
    pub short_open_profit_loss: Number,
    pub previous_session_long_quantity: i64,
    pub previous_session_short_quantity: i64,
    pub current_day_cost: Number,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "assetType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountsInstrument {
    CashEquivalent(AccountCashEquivalent),
    Equity(AccountEquity),
    FixedIncome(AccountFixedIncome),
    MutualFund(AccountMutualFund),
    Option(AccountOption),
    Index(AccountIndex),
    Currency(AccountCurrency),
    CollectiveInvestment(AccountCollectiveInvestment),
}

impl Default for AccountsInstrument {
    fn default() -> Self {
        Self::CashEquivalent(AccountCashEquivalent::default())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountCashEquivalent {
    #[serde(flatten)]
    pub accounts_base_instrument: AccountsBaseInstrument,

    #[serde(rename = "type")]
    pub type_field: AccountCashEquivalentType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountEquity {
    #[serde(flatten)]
    pub accounts_base_instrument: AccountsBaseInstrument,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountFixedIncome {
    #[serde(flatten)]
    pub accounts_base_instrument: AccountsBaseInstrument,

    pub maturity_date: chrono::DateTime<chrono::Utc>,
    pub factor: Number,
    pub variable_rate: Number,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountMutualFund {
    #[serde(flatten)]
    pub accounts_base_instrument: AccountsBaseInstrument,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountOption {
    #[serde(flatten)]
    pub accounts_base_instrument: AccountsBaseInstrument,

    /// xml: `OrderedMap` { "name": "optionDeliverables", "wrapped": true }
    pub option_deliverables: Vec<AccountAPIOptionDeliverable>,
    pub put_call: AccountOptionPutCall,
    pub option_multiplier: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: AccountOptionType,
    pub underlying_symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIndex {
    #[serde(flatten)]
    pub accounts_base_instrument: AccountsBaseInstrument,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountCurrency {
    #[serde(flatten)]
    pub accounts_base_instrument: AccountsBaseInstrument,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountCollectiveInvestment {
    #[serde(flatten)]
    pub accounts_base_instrument: AccountsBaseInstrument,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAPIOptionDeliverable {
    pub symbol: String,
    pub deliverable_units: Number,
    pub api_currency_type: Option<APICurrencyType>,
    pub asset_type: Option<AssetType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsBaseInstrument {
    pub cusip: String,
    pub symbol: String,
    pub description: Option<String>,
    pub instrument_id: i64,
    pub net_change: Option<Number>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum APICurrencyType {
    #[default]
    Usd,
    Cad,
    Eur,
    Jpy,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssetType {
    #[default]
    Equity,
    MutualFund,
    Option,
    Future,
    Forex,
    Index,
    CashEquivalent,
    FixedIncome,
    Product,
    Currency,
    CollectiveInvestment,
    #[serde(other)]
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountCashEquivalentType {
    #[default]
    SweepVehicle,
    Savings,
    MoneyMarketFund,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountOptionPutCall {
    #[default]
    Put,
    Call,
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountOptionType {
    #[default]
    Vanilla,
    Binary,
    Barrier,
    Unknown,
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_de_account() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Account_real.json"
        ));

        let val = serde_json::from_str::<Account>(json);
        tracing::debug!(?val);
        assert!(val.is_ok(), "Failed to deserialize: {:?}", val.err());
    }

    #[test]
    fn test_de_accounts() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Accounts_real.json"
        ));

        let val = serde_json::from_str::<Accounts>(json);
        tracing::debug!(?val);
        assert!(val.is_ok(), "Failed to deserialize: {:?}", val.err());
    }

    #[test]
    fn test_de_accounts2() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Accounts_real.json"
        ));

        let val = serde_json::from_str::<Accounts>(json);
        tracing::debug!(?val);
        assert!(val.is_ok(), "Failed to deserialize: {:?}", val.err());
    }
}
