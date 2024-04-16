use serde::Deserialize;
use serde::Serialize;

pub type Root = Vec<Transaction>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub activity_id: i64,
    pub time: String,
    pub user: UserDetails,
    pub description: String,
    pub account_number: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub status: String,
    pub sub_account: String,
    pub trade_date: String,
    pub settlement_date: String,
    pub position_id: i64,
    pub order_id: i64,
    pub net_amount: i64,
    pub activity_type: String,
    pub transfer_items: Vec<TransferItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
    pub cd_domain_id: String,
    pub login: String,
    #[serde(rename = "type")]
    pub type_field: String,
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
    pub amount: i64,
    pub cost: i64,
    pub price: i64,
    pub fee_type: String,
    pub position_effect: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum TransactionInstrument {
    Equity(TransactionEquity),
    Option(TransactionOption),
    Index(Index),
    MutualFund(TransactionMutualFund),
    CashEquivalent(TransactionCashEquivalent),
    FixedIncome(TransactionFixedIncome),
    Currency(Currency),
    CollectiveInvestment(CollectiveInvestment),
    Forex(Forex),
    Future(Future),
    Product(Product),
}

impl Default for TransactionInstrument {
    fn default() -> Self {
        Self::Equity(TransactionEquity::default())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionCashEquivalent {
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
pub struct CollectiveInvestment {
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
pub struct Currency {
    pub asset_type: Option<String>,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub instrument_id: i64,
    pub net_change: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEquity {
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
pub struct TransactionFixedIncome {
    pub asset_type: Option<String>,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub instrument_id: i64,
    pub net_change: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub maturity_date: chrono::DateTime<chrono::Utc>,
    pub factor: f64,
    pub multiplier: f64,
    pub variable_rate: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forex {
    pub asset_type: Option<String>,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub instrument_id: i64,
    pub net_change: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub base_currency: Currency,
    pub counter_currency: Currency,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Future {
    pub active_contract: bool,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
    pub last_trading_date: chrono::DateTime<chrono::Utc>,
    pub first_notice_date: chrono::DateTime<chrono::Utc>,
    pub multiplier: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub active_contract: bool,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMutualFund {
    pub asset_type: Option<String>,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub instrument_id: i64,
    pub net_change: i64,
    pub fund_family_name: String,
    pub fund_family_symbol: String,
    pub fund_group: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub exchange_cutoff_time: chrono::DateTime<chrono::Utc>,
    pub purchase_cutoff_time: chrono::DateTime<chrono::Utc>,
    pub redemption_cutoff_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionOption {
    pub asset_type: Option<String>,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub instrument_id: i64,
    pub net_change: i64,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
    pub option_deliverables: Vec<TransactionAPIOptionDeliverable>,
    pub option_premium_multiplier: i64,
    pub put_call: String,
    pub strike_price: f64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub underlying_symbol: String,
    pub underlying_cusip: String,
    pub deliverable: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAPIOptionDeliverable {
    pub root_symbol: String,
    pub strike_percent: i64,
    pub deliverable_number: i64,
    pub deliverable_units: f64,
    pub deliverable: String,
    pub asset_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub asset_type: Option<String>,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub instrument_id: i64,
    pub net_change: i64,
    #[serde(rename = "type")]
    pub type_field: String,
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
}
