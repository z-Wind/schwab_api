use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, TimestampMilliSeconds};

use crate::servicer::model::market_data::quote_response::equity::Fundamental;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualFundResponse {
    pub asset_main_type: String,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    pub fundamental: Fundamental,
    pub quote: QuoteMutualFund,
    pub reference: ReferenceMutualFund,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteMutualFund {
    #[serde(rename = "52WeekHigh")]
    pub n52week_high: f64,
    #[serde(rename = "52WeekLow")]
    pub n52week_low: f64,
    pub close_price: f64,
    #[serde(rename = "nAV")]
    pub n_av: i64,
    pub net_change: f64,
    pub net_percent_change: f64,
    pub security_status: String,
    pub total_volume: i64,
	#[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceMutualFund {
    pub cusip: String,
    pub description: String,
    pub exchange: String,
    pub exchange_name: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse/MutualFundResponse.json"
        ));

        let val = serde_json::from_str::<HashMap<String, MutualFundResponse>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}