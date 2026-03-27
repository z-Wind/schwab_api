use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;

use crate::Number;

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
    pub bond_price: Option<Number>,
    pub fundamental: Option<FundamentalInst>,
    pub instrument_info: Option<Instrument>,
    pub bond_instrument_info: Option<Bond>,

    /// writeOnly: true
    #[serde(rename = "type")]
    pub type_field: Option<InstrumentAssetType>,
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundamentalInst {
    pub symbol: String,
    pub high52: Number,
    pub low52: Number,
    pub dividend_amount: Number,
    pub dividend_yield: Number,
    #[serde(default, with = "custom_date_format")]
    pub dividend_date: Option<NaiveDateTime>,
    pub pe_ratio: Number,
    pub peg_ratio: Number,
    pub pb_ratio: Number,
    pub pr_ratio: Number,
    pub pcf_ratio: Number,
    #[serde(rename = "grossMarginTTM")]
    pub gross_margin_ttm: Number,
    #[serde(rename = "grossMarginMRQ")]
    pub gross_margin_mrq: Number,
    #[serde(rename = "netProfitMarginTTM")]
    pub net_profit_margin_ttm: Number,
    #[serde(rename = "netProfitMarginMRQ")]
    pub net_profit_margin_mrq: Number,
    #[serde(rename = "operatingMarginTTM")]
    pub operating_margin_ttm: Number,
    #[serde(rename = "operatingMarginMRQ")]
    pub operating_margin_mrq: Number,
    pub return_on_equity: Number,
    pub return_on_assets: Number,
    pub return_on_investment: Number,
    pub quick_ratio: Number,
    pub current_ratio: Number,
    pub interest_coverage: Number,
    pub total_debt_to_capital: Number,
    pub lt_debt_to_equity: Number,
    pub total_debt_to_equity: Number,
    #[serde(rename = "epsTTM")]
    pub eps_ttm: Number,
    #[serde(rename = "epsChangePercentTTM")]
    pub eps_change_percent_ttm: Number,
    pub eps_change_year: Number,
    pub eps_change: Number,
    pub rev_change_year: Number,
    #[serde(rename = "revChangeTTM")]
    pub rev_change_ttm: Number,
    pub rev_change_in: Number,
    pub shares_outstanding: Number,
    pub market_cap_float: Number,
    pub market_cap: Number,
    pub book_value_per_share: Number,
    pub short_int_to_float: Number,
    pub short_int_day_to_cover: Number,
    pub div_growth_rate3_year: Number,
    pub dividend_pay_amount: Number,
    #[serde(default, with = "custom_date_format")]
    pub dividend_pay_date: Option<NaiveDateTime>,
    pub beta: Number,
    pub vol1_day_avg: Number,
    pub vol10_day_avg: Number,
    pub vol3_month_avg: Number,
    pub avg10_days_volume: Number,
    pub avg1_day_volume: Number,
    pub avg3_month_volume: Number,
    #[serde(default, with = "custom_date_format")]
    pub declaration_date: Option<NaiveDateTime>,
    pub dividend_freq: i64,
    pub eps: Number,
    #[serde(default, with = "custom_date_format")]
    pub corpaction_date: Option<NaiveDateTime>,
    pub dtn_volume: Number,
    #[serde(default, with = "custom_date_format")]
    pub next_dividend_pay_date: Option<NaiveDateTime>,
    #[serde(default, with = "custom_date_format")]
    pub next_dividend_date: Option<NaiveDateTime>,
    pub fund_leverage_factor: Number,
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
    pub type_field: Option<InstrumentAssetType>,
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
    pub bond_price: Number,

    /// writeOnly: true
    #[serde(rename = "type")]
    pub type_field: Option<InstrumentAssetType>,
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

    #[serde(other)]
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
        if let Some(d) = date {
            let mut s = d.format(FORMAT).to_string();
            if !s.contains('.') {
                s.push_str(".0");
            }
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(ref s) if !s.is_empty() => NaiveDateTime::parse_from_str(s, FORMAT)
                .map(Some)
                .map_err(serde::de::Error::custom),
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_json_diff::{CompareMode, Config, NumericMode, assert_json_matches};
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
        assert!(val.is_ok(), "Failed to deserialize: {:?}", val.err());
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

        assert_json_matches!(
            val,
            json,
            Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat)
        );
    }
}
