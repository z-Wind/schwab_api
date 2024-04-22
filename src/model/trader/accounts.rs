use serde::Deserialize;
use serde::Serialize;

pub type Accounts = Vec<Account>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub securities_account: SecuritiesAccount,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesAccount {
    #[serde(rename = "type")]
    pub type_field: String,
    pub account_number: String,
    pub round_trips: i64,
    pub is_day_trader: bool,
    pub is_closing_only_restricted: bool,
    pub pfcb_flag: bool,
    pub positions: Vec<Position>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub short_quantity: i64,
    pub average_price: f64,
    pub current_day_profit_loss: i64,
    pub current_day_profit_loss_percentage: i64,
    pub long_quantity: i64,
    pub settled_long_quantity: i64,
    pub settled_short_quantity: i64,
    pub aged_quantity: i64,
    pub instrument: AccountsInstrument,
    pub market_value: i64,
    pub maintenance_requirement: i64,
    pub average_long_price: f64,
    pub average_short_price: f64,
    pub tax_lot_average_long_price: f64,
    pub tax_lot_average_short_price: f64,
    pub long_open_profit_loss: i64,
    pub short_open_profit_loss: i64,
    pub previous_session_long_quantity: i64,
    pub previous_session_short_quantity: i64,
    pub current_day_cost: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AccountsInstrument {
    Equity(AccountEquity),
    Option(AccountOption),
    CashEquivalent(AccountCashEquivalent),
    FixedIncome(AccountFixedIncome),
}

impl Default for AccountsInstrument {
    fn default() -> Self {
        Self::CashEquivalent(AccountCashEquivalent::default())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountCashEquivalent {
    pub asset_type: Option<String>,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub instrument_id: i64,
    pub net_change: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountEquity {
    pub asset_type: Option<String>,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub instrument_id: i64,
    pub net_change: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountFixedIncome {
    pub asset_type: Option<String>,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub instrument_id: i64,
    pub net_change: i64,
    pub maturity_date: chrono::DateTime<chrono::Utc>,
    pub factor: f64,
    pub variable_rate: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountOption {
    pub asset_type: Option<String>,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub instrument_id: i64,
    pub net_change: i64,
    pub option_deliverables: Vec<AccountAPIOptionDeliverable>,
    pub put_call: String,
    pub option_multiplier: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub underlying_symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAPIOptionDeliverable {
    pub symbol: i64,
    pub deliverable_units: f64,
    pub api_currency_type: String,
    pub asset_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_account() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Account.json"
        ));

        let val = serde_json::from_str::<Account>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_de_accounts() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/Accounts.json"
        ));

        let val = serde_json::from_str::<Accounts>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
