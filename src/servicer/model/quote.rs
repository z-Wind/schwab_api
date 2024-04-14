use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, TimestampMilliSeconds};
use std::collections::HashMap;

///
/// Quote
///
/// Deserialized through a `HashMap`<String, Quote>
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub(crate) enum Quote {
    Equity(QEquity),
    Index(QIndex),
    Option(QOption),
    Fund(QFund),
    General(QGeneral),
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QEquity {
    asset_type: String,
    symbol: String,
    description: String,
    bid_price: f64,
    bid_size: f64,
    bid_id: String,
    ask_price: f64,
    ask_size: f64,
    ask_id: String,
    last_price: f64,
    last_size: f64,
    last_id: String,
    open_price: f64,
    high_price: f64,
    low_price: f64,
    close_price: f64,
    net_change: f64,
    total_volume: u64,
    #[serde(rename = "quoteTimeInLong")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    quote_time: chrono::NaiveDateTime,
    #[serde(rename = "tradeTimeInLong")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    trade_time: chrono::NaiveDateTime,
    mark: f64,
    cusip: String,
    exchange: String,
    exchange_name: String,
    marginable: bool,
    shortable: bool,
    volatility: f64,
    digits: i64,
    #[serde(rename = "52WkHigh")]
    n52wk_high: f64,
    #[serde(rename = "52WkLow")]
    n52wk_low: f64,
    pe_ratio: f64,
    div_amount: f64,
    div_yield: f64,
    div_date: String,
    security_status: String,
    regular_market_last_price: f64,
    regular_market_last_size: f64,
    regular_market_net_change: f64,
    regular_market_trade_time_in_long: f64,
    net_percent_change_in_double: f64,
    mark_change_in_double: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl From<QEquity> for Quote {
    fn from(value: QEquity) -> Self {
        Quote::Equity(value)
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QIndex {
    asset_type: String,
    symbol: String,
    description: String,
    last_price: f64,
    open_price: f64,
    high_price: f64,
    low_price: f64,
    close_price: f64,
    net_change: f64,
    total_volume: u64,
    #[serde(rename = "tradeTimeInLong")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    trade_time: chrono::NaiveDateTime,
    exchange: String,
    exchange_name: String,
    digits: i64,
    #[serde(rename = "52WkHigh")]
    n52wk_high: f64,
    #[serde(rename = "52WkLow")]
    n52wk_low: f64,
    security_status: String,
    net_percent_change_in_double: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl From<QIndex> for Quote {
    fn from(value: QIndex) -> Self {
        Quote::Index(value)
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QOption {
    asset_type: String,
    symbol: String,
    description: String,
    bid_price: f64,
    bid_size: f64,
    ask_price: f64,
    ask_size: f64,
    last_price: f64,
    last_size: f64,
    open_price: f64,
    high_price: f64,
    low_price: f64,
    close_price: f64,
    net_change: f64,
    total_volume: u64,
    #[serde(rename = "quoteTimeInLong")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    quote_time: chrono::NaiveDateTime,
    #[serde(rename = "tradeTimeInLong")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    trade_time: chrono::NaiveDateTime,
    mark: f64,
    cusip: String,
    open_interest: f64,
    volatility: f64,
    money_intrinsic_value: f64,
    multiplier: f64,
    strike_price: f64,
    contract_type: String,
    underlying: String,
    time_value: f64,
    deliverables: String,
    delta: f64,
    gamma: f64,
    theta: f64,
    vega: f64,
    rho: f64,
    security_status: String,
    theoretical_option_value: f64,
    underlying_price: f64,
    uv_expiration_type: String,
    exchange: String,
    exchange_name: String,
    settlement_type: String,
    net_percent_change_in_double: f64,
    mark_change_in_double: f64,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    last_trading_day: chrono::NaiveDateTime,
    expiration_day: i64,
    expiration_month: i64,
    expiration_year: i64,
    days_to_expiration: i64,
    implied_yield: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl From<QOption> for Quote {
    fn from(value: QOption) -> Self {
        Quote::Option(value)
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QFund {
    asset_type: String,
    symbol: String,
    description: String,
    close_price: f64,
    net_change: f64,
    total_volume: u64,
    #[serde(rename = "tradeTimeInLong")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    trade_time: chrono::NaiveDateTime,
    cusip: String,
    exchange: String,
    exchange_name: String,
    digits: i64,
    #[serde(rename = "52WkHigh")]
    n52wk_high: f64,
    #[serde(rename = "52WkLow")]
    n52wk_low: f64,
    #[serde(rename = "nAV")]
    nav: f64,
    pe_ratio: f64,
    div_amount: f64,
    div_yield: f64,
    div_date: String,
    security_status: String,
    net_percent_change_in_double: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl From<QFund> for Quote {
    fn from(value: QFund) -> Self {
        Quote::Fund(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QGeneral {
    asset_type: String,
    symbol: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl From<QGeneral> for Quote {
    fn from(value: QGeneral) -> Self {
        Quote::General(value)
    }
}

impl Quote {
    #[allow(dead_code)]
    pub(crate) fn symbol(&self) -> &str {
        match self {
            Quote::Equity(quote) => &quote.symbol,
            Quote::Option(quote) => &quote.symbol,
            Quote::Fund(quote) => &quote.symbol,
            Quote::Index(quote) => &quote.symbol,
            Quote::General(quote) => &quote.symbol,
        }
    }

    pub(crate) fn market_price(&self) -> Option<f64> {
        match self {
            Quote::Equity(quote) => Some(quote.mark),
            Quote::Option(quote) => Some(quote.mark),
            Quote::Fund(quote) => Some(quote.nav),
            Quote::Index(quote) => Some(quote.last_price),
            Quote::General(_) => None,
        }
    }

    pub(crate) fn date(&self) -> chrono::NaiveDate {
        match self {
            Quote::Equity(quote) => quote.trade_time.date(),
            Quote::Option(quote) => quote.trade_time.date(),
            Quote::Fund(quote) => quote.trade_time.date(),
            Quote::Index(quote) => quote.trade_time.date(),
            Quote::General(_) => chrono::Local::now().naive_local().date(),
        }
    }

    pub(crate) fn open_price(&self) -> Option<f64> {
        match self {
            Quote::Equity(quote) => Some(quote.open_price),
            Quote::Option(quote) => Some(quote.open_price),
            Quote::Index(quote) => Some(quote.open_price),
            _ => None,
        }
    }

    pub(crate) fn high_price(&self) -> Option<f64> {
        match self {
            Quote::Equity(quote) => Some(quote.high_price),
            Quote::Option(quote) => Some(quote.high_price),
            Quote::Index(quote) => Some(quote.high_price),
            _ => None,
        }
    }

    pub(crate) fn low_price(&self) -> Option<f64> {
        match self {
            Quote::Equity(quote) => Some(quote.low_price),
            Quote::Option(quote) => Some(quote.low_price),
            Quote::Index(quote) => Some(quote.low_price),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn close_price(&self) -> Option<f64> {
        match self {
            Quote::Equity(quote) => Some(quote.close_price),
            Quote::Option(quote) => Some(quote.close_price),
            Quote::Fund(quote) => Some(quote.close_price),
            Quote::Index(quote) => Some(quote.close_price),
            Quote::General(_) => None,
        }
    }

    pub(crate) fn volume(&self) -> Option<u64> {
        match self {
            Quote::Equity(quote) => Some(quote.total_volume),
            Quote::Option(quote) => Some(quote.total_volume),
            Quote::Fund(quote) => Some(quote.total_volume),
            Quote::Index(quote) => Some(quote.total_volume),
            Quote::General(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_de_qequity() {
        let s = r#"{
            "VTI":{
                "assetType":"ETF",
                "assetMainType":"EQUITY",
                "cusip":"922908769",
                "assetSubType":"ETF",
                "symbol":"VTI",
                "description":"Vanguard Total Stock Market ETF",
                "bidPrice":195.61,
                "bidSize":100,
                "bidId":"P",
                "askPrice":196.0,
                "askSize":300,
                "askId":"P",
                "lastPrice":195.89,
                "lastSize":130500,
                "lastId":"P",
                "openPrice":195.5,
                "highPrice":196.95,
                "lowPrice":193.57,
                "bidTick":" ",
                "closePrice":192.46,
                "netChange":3.43,
                "totalVolume":4473782,
                "quoteTimeInLong":1678838381719,
                "tradeTimeInLong":1678838400003,
                "mark":195.89,
                "exchange":"p",
                "exchangeName":"PACIFIC",
                "marginable":true,
                "shortable":true,
                "volatility":0.0416,
                "digits":2,
                "52WkHigh":233.36,
                "52WkLow":174.84,
                "nAV":0.0,
                "peRatio":0.0,
                "divAmount":3.1833,
                "divYield":1.65,
                "divDate":"2022-12-22 00:00:00.000",
                "securityStatus":"Normal",
                "regularMarketLastPrice":195.89,
                "regularMarketLastSize":1305,
                "regularMarketNetChange":3.43,
                "regularMarketTradeTimeInLong":1678838400003,
                "netPercentChangeInDouble":1.7822,
                "markChangeInDouble":3.43,
                "markPercentChangeInDouble":1.7822,
                "regularMarketPercentChangeInDouble":1.7822,
                "delayed":false,
                "realtimeEntitled":true
            },
            "VBR":{
                "assetType":"ETF",
                "assetMainType":"EQUITY",
                "cusip":"922908611",
                "assetSubType":"ETF",
                "symbol":"VBR",
                "description":"Vanguard Small-Cap Value ETF",
                "bidPrice":155.0,
                "bidSize":100,
                "bidId":"P",
                "askPrice":165.1,
                "askSize":200,
                "askId":"P",
                "lastPrice":156.81,
                "lastSize":700,
                "lastId":"P",
                "openPrice":158.5,
                "highPrice":159.57,
                "lowPrice":155.23,
                "bidTick":" ",
                "closePrice":154.23,
                "netChange":2.58,
                "totalVolume":439774,
                "quoteTimeInLong":1678834343624,
                "tradeTimeInLong":1678838400003,
                "mark":156.81,
                "exchange":"p",
                "exchangeName":"PACIFIC",
                "marginable":true,
                "shortable":true,
                "volatility":0.0914,
                "digits":2,
                "52WkHigh":181.16,
                "52WkLow":142.4801,
                "nAV":0.0,
                "peRatio":0.0,
                "divAmount":3.2283,
                "divYield":2.09,
                "divDate":"2022-12-22 00:00:00.000",
                "securityStatus":"Normal",
                "regularMarketLastPrice":156.81,
                "regularMarketLastSize":7,
                "regularMarketNetChange":2.58,
                "regularMarketTradeTimeInLong":1678838400003,
                "netPercentChangeInDouble":1.6728,
                "markChangeInDouble":2.58,
                "markPercentChangeInDouble":1.6728,
                "regularMarketPercentChangeInDouble":1.6728,
                "delayed":false,
                "realtimeEntitled":true
            }
        }"#;
        let val = serde_json::from_str::<HashMap<String, QEquity>>(s);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
