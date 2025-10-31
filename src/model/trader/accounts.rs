use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

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
    pub is_day_trader: bool,
    /// default: false
    pub is_closing_only_restricted: bool,
    /// default: false
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
    pub accrued_interest: f64,
    pub available_funds_non_marginable_trade: f64,
    pub bond_value: f64,
    pub buying_power: f64,
    pub cash_balance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_available_for_trading: Option<f64>,
    pub cash_receipts: f64,
    pub day_trading_buying_power: f64,
    pub day_trading_buying_power_call: f64,
    pub day_trading_equity_call: f64,
    pub equity: f64,
    pub equity_percentage: f64,
    pub liquidation_value: f64,
    pub long_margin_value: f64,
    pub long_option_market_value: f64,
    pub long_stock_value: f64,
    pub maintenance_call: f64,
    pub maintenance_requirement: f64,
    pub margin: f64,
    pub margin_equity: f64,
    pub money_market_fund: f64,
    pub mutual_fund_value: f64,
    pub reg_t_call: f64,
    pub short_margin_value: f64,
    pub short_option_market_value: f64,
    pub short_stock_value: f64,
    pub total_cash: f64,
    pub is_in_call: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsettled_cash: Option<f64>,
    pub pending_deposits: f64,
    pub margin_balance: f64,
    pub short_balance: f64,
    pub account_value: f64,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginBalance {
    pub available_funds: Option<f64>,
    pub available_funds_non_marginable_trade: Option<f64>,
    pub buying_power: Option<f64>,
    pub buying_power_non_marginable_trade: Option<f64>,
    pub day_trading_buying_power: Option<f64>,
    pub day_trading_buying_power_call: Option<f64>,
    pub equity: Option<f64>,
    pub equity_percentage: Option<f64>,
    pub long_margin_value: Option<f64>,
    pub maintenance_call: Option<f64>,
    pub maintenance_requirement: Option<f64>,
    pub margin_balance: Option<f64>,
    pub reg_t_call: Option<f64>,
    pub short_balance: Option<f64>,
    pub short_margin_value: Option<f64>,
    pub sma: Option<f64>,
    pub is_in_call: Option<bool>,
    pub stock_buying_power: Option<f64>,
    pub option_buying_power: Option<f64>,
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
    pub accrued_interest: f64,
    pub cash_available_for_trading: f64,
    pub cash_available_for_withdrawal: f64,
    pub cash_balance: f64,
    pub bond_value: f64,
    pub cash_receipts: f64,
    pub liquidation_value: f64,
    pub long_option_market_value: f64,
    pub long_stock_value: f64,
    pub money_market_fund: f64,
    pub mutual_fund_value: f64,
    pub short_option_market_value: f64,
    pub short_stock_value: f64,
    pub is_in_call: bool,
    pub unsettled_cash: f64,
    pub cash_debit_call_value: f64,
    pub pending_deposits: f64,
    pub account_value: f64,
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashBalance {
    pub cash_available_for_trading: f64,
    pub cash_available_for_withdrawal: f64,
    pub cash_call: Option<f64>,
    pub long_non_marginable_market_value: Option<f64>,
    pub total_cash: Option<f64>,
    pub cash_debit_call_value: Option<f64>,
    pub unsettled_cash: Option<f64>,

    // not in schema
    pub accrued_interest: Option<f64>,
    pub cash_balance: Option<f64>,
    pub cash_receipts: Option<f64>,
    pub long_option_market_value: Option<f64>,
    pub liquidation_value: Option<f64>,
    pub long_market_value: Option<f64>,
    pub money_market_fund: Option<f64>,
    pub savings: Option<f64>,
    pub short_market_value: Option<f64>,
    pub pending_deposits: Option<f64>,
    pub mutual_fund_value: Option<f64>,
    pub bond_value: Option<f64>,
    pub short_option_market_value: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub short_quantity: f64,
    pub average_price: f64,
    pub current_day_profit_loss: f64,
    pub current_day_profit_loss_percentage: i64,
    pub long_quantity: f64,
    pub settled_long_quantity: f64,
    pub settled_short_quantity: f64,
    pub aged_quantity: f64,
    pub instrument: AccountsInstrument,
    pub market_value: f64,
    pub maintenance_requirement: f64,
    pub average_long_price: f64,
    pub average_short_price: f64,
    pub tax_lot_average_long_price: f64,
    pub tax_lot_average_short_price: f64,
    pub long_open_profit_loss: f64,
    pub short_open_profit_loss: f64,
    pub previous_session_long_quantity: i64,
    pub previous_session_short_quantity: i64,
    pub current_day_cost: f64,
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
    pub factor: f64,
    pub variable_rate: f64,
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
    pub put_call: AccountOptionPullCall,
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
    pub deliverable_units: f64,
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
    pub net_change: Option<f64>,
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
pub enum AccountOptionPullCall {
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
    use super::*;

    #[test]
    fn test_de_account() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Account_real.json"
        ));

        let val = serde_json::from_str::<Account>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_de_accounts() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Accounts_real.json"
        ));

        let val = serde_json::from_str::<Accounts>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_de_accounts2() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Accounts_real.json"
        ));

        let val = serde_json::from_str::<Accounts>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
