use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instruments {
    pub instruments: Vec<InstrumentResponse>,
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentResponse {
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub exchange: String,
    pub asset_type: InstrumentAssetType,
    pub bond_factor: Option<String>,
    pub bond_multiplier: Option<String>,
    pub bond_price: Option<f64>,
    pub fundamental: Option<FundamentalInst>,
    pub instrument_info: Option<Instrument>,
    pub bond_instrument_info: Option<Bond>,

    /// writeOnly: true
    #[serde(rename = "type")]
    pub type_filed: Option<InstrumentAssetType>,
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundamentalInst {
    pub symbol: String,
    pub high52: f64,
    pub low52: f64,
    pub dividend_amount: f64,
    pub dividend_yield: f64,
    #[serde(default, with = "custom_date_format")]
    pub dividend_date: Option<NaiveDateTime>,
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
    #[serde(default, with = "custom_date_format")]
    pub dividend_pay_date: Option<NaiveDateTime>,
    pub beta: f64,
    pub vol1_day_avg: f64,
    pub vol10_day_avg: f64,
    pub vol3_month_avg: f64,
    pub avg10_days_volume: f64,
    pub avg1_day_volume: f64,
    pub avg3_month_volume: f64,
    #[serde(default, with = "custom_date_format")]
    pub declaration_date: Option<NaiveDateTime>,
    pub dividend_freq: i64,
    pub eps: f64,
    #[serde(default, with = "custom_date_format")]
    pub corpaction_date: Option<NaiveDateTime>,
    pub dtn_volume: f64,
    #[serde(default, with = "custom_date_format")]
    pub next_dividend_pay_date: Option<NaiveDateTime>,
    #[serde(default, with = "custom_date_format")]
    pub next_dividend_date: Option<NaiveDateTime>,
    pub fund_leverage_factor: f64,
    pub fund_strategy: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub exchange: String,
    pub asset_type: InstrumentAssetType,

    /// writeOnly: true
    #[serde(rename = "type")]
    pub type_filed: Option<InstrumentAssetType>,
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bond {
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub exchange: String,
    pub asset_type: InstrumentAssetType,
    pub bond_factor: String,
    pub bond_multiplier: String,
    pub bond_price: f64,

    /// writeOnly: true
    #[serde(rename = "type")]
    pub type_filed: Option<InstrumentAssetType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstrumentAssetType {
    Bond,
    Equity,
    Etf,
    Extended,
    Forex,
    Future,
    FutureOption,
    Fundamental,
    Index,
    Indicator,
    MutualFund,
    Option,
    Unknown,
}

mod custom_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f";

    #[allow(clippy::ref_option)]
    pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => {
                let s = date.format(FORMAT).to_string();
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;

        Ok(Some(date))
    }
}

#[cfg(test)]
mod tests {
    use assert_json_diff::{CompareMode, Config, NumericMode, assert_json_matches_no_panic};
    use test_log::test;

    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/Instruments.json"
        ));

        let val = serde_json::from_str::<Instruments>(json);
        tracing::debug!(?val);
        assert!(val.is_ok());
    }

    #[test]
    fn test_serde_real() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/Instruments_real.json"
        ));
        let json: serde_json::Value = serde_json::from_str(json).unwrap();

        let val = serde_json::from_value::<Instruments>(json.clone()).unwrap();
        tracing::debug!(?val);

        let message = assert_json_matches_no_panic(
            &val,
            &json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat),
        )
        .unwrap_err();

        let re =
            regex::Regex::new(r"(?:json atoms at path.*Date.*are not equal.*\n.*\n.*\n.*\n.*)")
                .unwrap();
        let message = re.replace_all(&message, "");
        let message = message.trim();
        tracing::debug!(%message);
        assert_eq!(message, "");
    }
}
