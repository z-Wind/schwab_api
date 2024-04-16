use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, TimestampMilliSeconds};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instruments {
    pub instruments: Vec<InstrumentResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentResponse {
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub exchange: String,
    pub asset_type: String,
    pub bond_factor: Option<String>,
    pub bond_multiplier: Option<String>,
    pub bond_price: Option<f64>,
    pub fundamental: Option<FundamentalInst>,
    pub instrument_info: Option<Instrument>,
    pub bond_instrument_info: Option<Bond>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundamentalInst {
    pub symbol: String,
    pub high52: f64,
    pub low52: f64,
    pub dividend_amount: f64,
    pub dividend_yield: f64,
    pub dividend_date: String,
    pub pe_ratio: f64,
    pub peg_ratio: f64,
    pub pb_ratio: f64,
    pub pr_ratio: f64,
    pub pcf_ratio: f64,
	#[serde(rename = "grossMarginTTM")]
    pub gross_margin_ttm: f64,
	#[serde(rename = "grossMarginMRQ")]
    pub gross_margin_mrq: f64,
	#[serde(rename = "netProfitMarginTTM")]
    pub net_profit_margin_ttm: f64,
	#[serde(rename = "netProfitMarginMRQ")]
    pub net_profit_margin_mrq: f64,
	#[serde(rename = "operatingMarginTTM")]
    pub operating_margin_ttm: f64,
	#[serde(rename = "operatingMarginMRQ")]
    pub operating_margin_mrq: f64,
    pub return_on_equity: f64,
    pub return_on_assets: f64,
    pub return_on_investment: f64,
    pub quick_ratio: f64,
    pub current_ratio: f64,
    pub interest_coverage: f64,
    pub total_debt_to_capital: f64,
    pub lt_debt_to_equity: f64,
    pub total_debt_to_equity: f64,
    #[serde(rename = "epsTTM")]
	pub eps_ttm: f64,
    #[serde(rename = "epsChangePercentTTM")]
	pub eps_change_percent_ttm: f64,
    pub eps_change_year: f64,
    pub eps_change: f64,
    pub rev_change_year: f64,
	#[serde(rename = "revChangeTTM")]
    pub rev_change_ttm: f64,
    pub rev_change_in: f64,
    pub shares_outstanding: f64,
    pub market_cap_float: f64,
    pub market_cap: f64,
    pub book_value_per_share: f64,
    pub short_int_to_float: f64,
    pub short_int_day_to_cover: f64,
    pub div_growth_rate3_year: f64,
    pub dividend_pay_amount: f64,
    pub dividend_pay_date: String,
    pub beta: f64,
    pub vol1_day_avg: f64,
    pub vol10_day_avg: f64,
    pub vol3_month_avg: f64,
    pub avg10_days_volume: i64,
    pub avg1_day_volume: i64,
    pub avg3_month_volume: i64,
    pub declaration_date: String,
    pub dividend_freq: i64,
    pub eps: f64,
    pub corpaction_date: String,
    pub dtn_volume: i64,
    pub next_dividend_pay_date: String,
    pub next_dividend_date: String,
    pub fund_leverage_factor: f64,
    pub fund_strategy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub exchange: String,
    pub asset_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bond {
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub exchange: String,
    pub asset_type: String,
    pub bond_factor: String,
    pub bond_multiplier: String,
    pub bond_price: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/Instruments.json"
        ));

        let val = serde_json::from_str::<Instruments>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
