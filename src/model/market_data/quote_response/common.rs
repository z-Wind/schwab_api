use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};

use crate::Number;

/// Quote info of Common security
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonResponse {
    /// example: 1234567890
    ///
    /// SSID of instrument
    pub ssid: i64,

    /// Symbol of instrument
    pub symbol: String,

    /// example: true
    ///
    /// is quote realtime
    pub realtime: bool,

    pub quote: QuoteCommon,

    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Quote data of Equity security
#[serde_as]
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteCommon {
    /// example: 126.27
    ///
    /// Previous day's closing price
    pub close_price: Number,

    /// example: -0.04
    ///
    /// Current Last-Prev Close
    pub net_change: Number,

    /// example: Normal
    ///
    /// Status of security
    pub security_status: String,

    /// example: 20171188
    ///
    /// Aggregated shares traded throughout the day, including pre/post market hours.
    pub total_volume: u64,

    /// example: 1621376731304
    ///
    /// Last trade time in milliseconds since Epoch
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub trade_time: chrono::DateTime<chrono::Utc>,

    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};
    use test_log::test;

    use super::*;

    #[test]
    fn test_de() {
        let test_files = vec![
            "EquityResponse.json",
            "ForexResponse.json",
            "FutureResponse.json",
            "IndexResponse.json",
            "MutualFundResponse.json",
            "OptionResponse.json",
        ];

        let base_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/model/MarketData/QuoteResponse");

        for file_name in test_files {
            let file_path = base_path.join(file_name);

            // Load the actual file content as a string
            let json_content = std::fs::read_to_string(&file_path)
                .unwrap_or_else(|_| panic!("Failed to read file: {file_path:?}"));

            let val = serde_json::from_str::<HashMap<String, CommonResponse>>(&json_content);

            tracing::debug!(file = ?file_name, result = ?val);

            assert!(
                val.is_ok(),
                "Failed to deserialize file {:?}: {:?}",
                file_name,
                val.err()
            );
        }
    }
}
