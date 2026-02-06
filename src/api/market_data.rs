//! APIs to access Market Data
//! [API Documentation](https://developer.schwab.com/products/trader-api--individual/details/specifications/Market%20Data%20Production)

use reqwest::{Client, RequestBuilder, StatusCode};
use std::collections::HashMap;

use super::endpoints;
use super::parameter::{
    ContractType, Entitlement, FrequencyType, Market, Month, OptionChainStrategy, PeriodType,
    Projection, QuoteField, SortAttribute,
};
use super::save_raw_json;
use crate::api::Error;
use crate::model;

/// Get Quotes by list of symbols.
#[derive(Debug)]
pub struct GetQuotesRequest {
    req: RequestBuilder,

    symbols: Vec<String>,

    /// Request for subset of data by passing coma separated list of root nodes, possible root nodes are `quote`, `fundamental`, `extended`, `reference`, `regular`.
    ///
    /// Sending quote, fundamental in request will return quote and fundamental data in response.
    ///
    /// Dont send this attribute for full response.
    ///
    /// Default value : all
    fields: Option<Vec<QuoteField>>,

    /// Include indicative symbol quotes for all ETF symbols in request.
    ///
    /// If ETF symbol ABC is in request and indicative=true API will return quotes for ABC and its corresponding indicative quote for $ABC.IV
    indicative: Option<bool>,
}

impl GetQuotesRequest {
    fn endpoint() -> endpoints::EndpointQuote {
        endpoints::EndpointQuote::Quotes
    }

    pub(crate) fn new(client: &Client, access_token: String, symbols: Vec<String>) -> Self {
        let req = client.get(Self::endpoint().url()).bearer_auth(access_token);
        Self::new_with(req, symbols)
    }

    fn new_with(req: RequestBuilder, symbols: Vec<String>) -> Self {
        Self {
            req,
            symbols,
            fields: None,
            indicative: None,
        }
    }

    /// Request for subset of data by passing coma separated list of root nodes, possible root nodes are `quote`, `fundamental`, `extended`, `reference`, `regular`.
    ///
    /// Sending quote, fundamental in request will return quote and fundamental data in response.
    ///
    /// Dont send this attribute for full response.
    ///
    /// Default value : `all`
    pub fn fields(&mut self, val: Vec<QuoteField>) -> &mut Self {
        self.fields = Some(val);
        self
    }

    /// Include indicative symbol quotes for all ETF symbols in request.
    ///
    /// If ETF symbol ABC is in request and indicative=true API will return quotes for ABC and its corresponding indicative quote for $ABC.IV
    pub fn indicative(&mut self, val: bool) -> &mut Self {
        self.indicative = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req.query(&[("symbols", self.symbols.join(","))]);

        if let Some(ref fields) = self.fields {
            let field_strs: Vec<String> = fields
                .into_iter()
                .filter_map(|f| match f {
                    QuoteField::Extra(s) => Some(s.clone()),
                    _ => serde_json::to_value(&f)
                        .ok()
                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                })
                .collect();

            if !field_strs.is_empty() {
                req = req.query(&[("fields", field_strs.join(","))]);
            }
        }

        if let Some(indicative) = self.indicative {
            req = req.query(&[("indicative", indicative.to_string())]);
        }

        tracing::debug!(
            "request built with {} fields",
            self.fields.as_ref().map_or(0, |v| v.len())
        );

        req
    }

    pub async fn send(self) -> Result<HashMap<String, model::QuoteResponse>, Error> {
        let req = self.build();
        let rsp = req.send().await?;
        let status = rsp.status();

        let body_text = rsp.text().await?;

        if status != StatusCode::OK {
            let error_response = serde_json::from_str(&body_text).map_err(|e| {
                save_raw_json("log", "ErrorResponse", &body_text);
                Error::from(e)
            })?;

            return Err(Error::Response(error_response));
        }

        let map: model::QuoteResponseMap = serde_json::from_str(&body_text).map_err(|e| {
            save_raw_json("log", "QuoteResponseMap", &body_text);
            Error::from(e)
        })?;

        if let Some(e) = map.errors {
            return Err(Error::Quote(e));
        }

        Ok(map.responses)
    }
}

/// Get Quote by single symbol.
#[derive(Debug)]
pub struct GetQuoteRequest {
    req: RequestBuilder,

    symbol: String,

    /// Request for subset of data by passing coma separated list of root nodes, possible root nodes are `quote`, `fundamental`, `extended`, `reference`, `regular`.
    ///
    /// Sending quote, fundamental in request will return quote and fundamental data in response.
    ///
    /// Dont send this attribute for full response.
    ///
    /// Default value : `all`
    fields: Option<Vec<QuoteField>>,
}

impl GetQuoteRequest {
    fn endpoint(symbol_id: String) -> endpoints::EndpointQuote {
        endpoints::EndpointQuote::Quote { symbol_id }
    }

    pub(crate) fn new(client: &Client, access_token: String, symbol: String) -> Self {
        let req = client
            .get(Self::endpoint(symbol.clone()).url())
            .bearer_auth(access_token);
        Self::new_with(req, symbol)
    }

    fn new_with(req: RequestBuilder, symbol: String) -> Self {
        Self {
            req,
            symbol,
            fields: None,
        }
    }

    /// Request for subset of data by passing coma separated list of root nodes, possible root nodes are `quote`, `fundamental`, `extended`, `reference`, `regular`.
    ///
    /// Sending quote, fundamental in request will return quote and fundamental data in response.
    ///
    /// Dont send this attribute for full response.
    ///
    /// Default value : `all`
    pub fn fields(&mut self, val: Vec<QuoteField>) -> &mut Self {
        self.fields = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req;

        if let Some(ref fields) = self.fields {
            let field_strs: Vec<String> = fields
                .into_iter()
                .filter_map(|f| match f {
                    QuoteField::Extra(s) => Some(s.clone()),
                    _ => serde_json::to_value(&f)
                        .ok()
                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                })
                .collect();

            if !field_strs.is_empty() {
                req = req.query(&[("fields", field_strs.join(","))]);
            }
        }

        tracing::debug!(
            "request built with {} fields",
            self.fields.as_ref().map_or(0, |v| v.len())
        );

        req
    }

    pub async fn send(self) -> Result<model::QuoteResponse, Error> {
        let symbol = self.symbol.clone();

        let req = self.build();
        let rsp = req.send().await.map_err(|e| {
            tracing::error!(error = %e, "network request failed");
            e
        })?;

        let status = rsp.status();
        let body_text = rsp.text().await.map_err(|e| {
            tracing::error!(error = %e, "failed to read response body");
            e
        })?;

        if status != StatusCode::OK {
            tracing::warn!(%status, "received non-OK response from server");

            let error_response = serde_json::from_str(&body_text).map_err(|e| {
                save_raw_json("log", "ErrorResponse", &body_text);
                e
            })?;

            return Err(Error::Response(error_response));
        }

        let mut map: model::QuoteResponseMap = serde_json::from_str(&body_text).map_err(|e| {
            save_raw_json("log", "QuoteResponseMap", &body_text);
            tracing::error!(error = %e, "json parse failed");
            e
        })?;

        if let Some(e) = map.errors {
            return Err(Error::Quote(e));
        }

        let val = map.responses.remove(&symbol).ok_or_else(|| {
            tracing::error!("requested symbol missing in response map");
            Error::Quote(model::QuoteError {
                invalid_symbols: Some(vec![symbol]),
                invalid_cusips: None,
                invalid_ssids: None,
            })
        })?;

        Ok(val)
    }
}

/// Get option chain for an optionable Symbol
#[derive(Debug)]
pub struct GetOptionChainsRequest {
    req: RequestBuilder,

    symbol: String,

    /// Contract Type
    ///
    /// Available values : `CALL`, `PUT`, `ALL`
    contract_type: Option<ContractType>,

    /// The Number of strikes to return above or below the at-the-money price
    strike_count: Option<i64>,

    /// Underlying quotes to be included
    include_underlying_quote: Option<bool>,

    /// `OptionChain` strategy.
    ///
    /// Default is `SINGLE`.
    ///
    /// `ANALYTICAL` allows the use of [`Self::volatility`], [`Self::underlying_price`], [`Self::interest_rate`], and [`Self::days_to_expiration`] params to calculate theoretical values.
    ///
    /// Available values : `SINGLE`, `ANALYTICAL`, `COVERED`, `VERTICAL`, `CALENDAR`, `STRANGLE`, `STRADDLE`, `BUTTERFLY`, `CONDOR`, `DIAGONAL`, `COLLAR`, `ROLL`
    strategy: Option<OptionChainStrategy>,

    /// Strike interval for spread strategy chains (see [`Self::strategy`] param)
    interval: Option<f64>,

    /// Strike Price
    strike: Option<f64>,

    /// Range(ITM/NTM/OTM etc.)
    range: Option<String>,

    /// From date
    // pattern: yyyy-MM-dd
    from_date: Option<chrono::NaiveDate>,

    /// To date
    // pattern: yyyy-MM-dd
    to_date: Option<chrono::NaiveDate>,

    /// Volatility to use in calculations.
    ///
    /// Applies only to `ANALYTICAL` strategy chains (see [`Self::strategy`] param)
    volatility: Option<f64>,

    /// Underlying price to use in calculations.
    ///
    /// Applies only to `ANALYTICAL` strategy chains (see [`Self::strategy`] param)
    underlying_price: Option<f64>,

    /// Interest rate to use in calculations.
    ///
    /// Applies only to `ANALYTICAL` strategy chains (see [`Self::strategy`] param)
    interest_rate: Option<f64>,

    /// Days to expiration to use in calculations.
    ///
    /// Applies only to `ANALYTICAL` strategy chains (see [`Self::strategy`] param)
    days_to_expiration: Option<i64>,

    /// Expiration month
    ///
    /// Available values : `JAN`, `FEB`, `MAR`, `APR`, `MAY`, `JUN`, `JUL`, `AUG`, `SEP`, `OCT`, `NOV`, `DEC`, `ALL`
    exp_month: Option<Month>,

    /// Option Type
    option_type: Option<String>,

    /// Applicable only if its retail token, entitlement of client PP-PayingPro, NP-NonPro and PN-NonPayingPro
    ///
    /// Available values : `PN`, `NP`, `PP`
    entitlement: Option<Entitlement>,
}

impl GetOptionChainsRequest {
    fn endpoint() -> endpoints::EndpointOptionChain {
        endpoints::EndpointOptionChain::Chains
    }

    pub(crate) fn new(client: &Client, access_token: String, symbol: String) -> Self {
        let req = client.get(Self::endpoint().url()).bearer_auth(access_token);
        Self::new_with(req, symbol)
    }

    fn new_with(req: RequestBuilder, symbol: String) -> Self {
        Self {
            req,
            symbol,
            contract_type: None,
            strike_count: None,
            include_underlying_quote: None,
            strategy: None,
            interval: None,
            strike: None,
            range: None,
            from_date: None,
            to_date: None,
            volatility: None,
            underlying_price: None,
            interest_rate: None,
            days_to_expiration: None,
            exp_month: None,
            option_type: None,
            entitlement: None,
        }
    }

    /// Contract Type
    /// Available values : CALL, PUT, ALL
    pub fn contract_type(&mut self, val: ContractType) -> &mut Self {
        self.contract_type = Some(val);
        self
    }

    /// The Number of strikes to return above or below the at-the-money price
    pub fn strike_count(&mut self, val: i64) -> &mut Self {
        self.strike_count = Some(val);
        self
    }

    /// Underlying quotes to be included
    pub fn include_underlying_quote(&mut self, val: bool) -> &mut Self {
        self.include_underlying_quote = Some(val);
        self
    }

    /// `OptionChain` strategy.
    ///
    /// Default is `SINGLE`.
    ///
    /// `ANALYTICAL` allows the use of [`Self::volatility`], [`Self::underlying_price`], [`Self::interest_rate`], and [`Self::days_to_expiration`] params to calculate theoretical values.
    ///
    /// Available values : `SINGLE`, `ANALYTICAL`, `COVERED`, `VERTICAL`, `CALENDAR`, `STRANGLE`, `STRADDLE`, `BUTTERFLY`, `CONDOR`, `DIAGONAL`, `COLLAR`, `ROLL`
    pub fn strategy(&mut self, val: OptionChainStrategy) -> &mut Self {
        self.strategy = Some(val);
        self
    }

    /// Strike interval for spread strategy chains (see [`Self::strategy`] param)
    pub fn interval(&mut self, val: f64) -> &mut Self {
        self.interval = Some(val);
        self
    }

    /// Strike Price
    pub fn strike(&mut self, val: f64) -> &mut Self {
        self.strike = Some(val);
        self
    }

    /// Range(ITM/NTM/OTM etc.)
    pub fn range(&mut self, val: String) -> &mut Self {
        self.range = Some(val);
        self
    }

    #[allow(clippy::wrong_self_convention)]
    /// From date
    pub fn from_date(&mut self, val: chrono::NaiveDate) -> &mut Self {
        self.from_date = Some(val);
        self
    }

    #[allow(clippy::wrong_self_convention)]
    /// To date
    pub fn to_date(&mut self, val: chrono::NaiveDate) -> &mut Self {
        self.to_date = Some(val);
        self
    }

    /// Volatility to use in calculations.
    ///
    /// Applies only to `ANALYTICAL` strategy chains (see [`Self::strategy`] param)
    pub fn volatility(&mut self, val: f64) -> &mut Self {
        self.volatility = Some(val);
        self
    }

    /// Underlying price to use in calculations.
    ///
    /// Applies only to `ANALYTICAL` strategy chains (see [`Self::strategy`] param)
    pub fn underlying_price(&mut self, val: f64) -> &mut Self {
        self.underlying_price = Some(val);
        self
    }

    /// Interest rate to use in calculations.
    ///
    /// Applies only to `ANALYTICAL` strategy chains (see [`Self::strategy`] param)
    pub fn interest_rate(&mut self, val: f64) -> &mut Self {
        self.interest_rate = Some(val);
        self
    }

    /// Days to expiration to use in calculations.
    ///
    /// Applies only to `ANALYTICAL` strategy chains (see [`Self::strategy`] param)
    pub fn days_to_expiration(&mut self, val: i64) -> &mut Self {
        self.days_to_expiration = Some(val);
        self
    }

    /// Expiration month
    ///
    /// Available values : `JAN`, `FEB`, `MAR`, `APR`, `MAY`, `JUN`, `JUL`, `AUG`, `SEP`, `OCT`, `NOV`, `DEC`, `ALL`
    pub fn exp_month(&mut self, val: Month) -> &mut Self {
        self.exp_month = Some(val);
        self
    }

    /// Option Type
    pub fn option_type(&mut self, val: String) -> &mut Self {
        self.option_type = Some(val);
        self
    }

    /// Applicable only if its retail token, entitlement of client PP-PayingPro, NP-NonPro and PN-NonPayingPro
    ///
    /// Available values : `PN`, `NP`, `PP`
    pub fn entitlement(&mut self, val: Entitlement) -> &mut Self {
        self.entitlement = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req.query(&[("symbol", self.symbol)]);
        if let Some(x) = self.contract_type {
            req = req.query(&[("contractType", x)]);
        }
        if let Some(x) = self.strike_count {
            req = req.query(&[("strikeCount", x)]);
        }
        if let Some(x) = self.include_underlying_quote {
            req = req.query(&[("includeUnderlyingQuote", x)]);
        }
        if let Some(x) = self.strategy {
            req = req.query(&[("strategy", x)]);
        }
        if let Some(x) = self.interval {
            req = req.query(&[("interval", x)]);
        }
        if let Some(x) = self.strike {
            req = req.query(&[("strike", x)]);
        }
        if let Some(x) = self.range {
            req = req.query(&[("range", x)]);
        }
        if let Some(x) = self.from_date {
            req = req.query(&[("fromDate", x)]);
        }
        if let Some(x) = self.to_date {
            req = req.query(&[("toDate", x)]);
        }
        if let Some(x) = self.volatility {
            req = req.query(&[("volatility", x)]);
        }
        if let Some(x) = self.underlying_price {
            req = req.query(&[("underlyingPrice", x)]);
        }
        if let Some(x) = self.interest_rate {
            req = req.query(&[("interestRate", x)]);
        }
        if let Some(x) = self.days_to_expiration {
            req = req.query(&[("daysToExpiration", x)]);
        }
        if let Some(x) = self.exp_month {
            req = req.query(&[("expMonth", x)]);
        }
        if let Some(x) = self.option_type {
            req = req.query(&[("optionType", x)]);
        }
        if let Some(x) = self.entitlement {
            req = req.query(&[("entitlement", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<model::OptionChain, Error> {
        let req = self.build();
        let rsp = req.send().await?;
        let status = rsp.status();

        let body_text = rsp.text().await?;

        if status != StatusCode::OK {
            let error_response = serde_json::from_str(&body_text).map_err(|e| {
                save_raw_json("log", "ErrorResponse", &body_text);
                Error::from(e)
            })?;

            return Err(Error::Response(error_response));
        }

        serde_json::from_str(&body_text).map_err(|e| {
            save_raw_json("log", "OptionChain", &body_text);
            Error::from(e)
        })
    }
}

/// Get option expiration chain for an optionable symbol
#[derive(Debug)]
pub struct GetOptionExpirationChainRequest {
    req: RequestBuilder,

    symbol: String,
}

impl GetOptionExpirationChainRequest {
    fn endpoint() -> endpoints::EndpointOptionExpirationChain {
        endpoints::EndpointOptionExpirationChain::ExpirationChain
    }

    pub(crate) fn new(client: &Client, access_token: String, symbol: String) -> Self {
        let req: RequestBuilder = client.get(Self::endpoint().url()).bearer_auth(access_token);
        Self::new_with(req, symbol)
    }

    fn new_with(req: RequestBuilder, symbol: String) -> Self {
        Self { req, symbol }
    }

    fn build(self) -> RequestBuilder {
        self.req.query(&[("symbol", self.symbol)])
    }

    pub async fn send(self) -> Result<model::ExpirationChain, Error> {
        let req = self.build();
        let rsp = req.send().await?;
        let status = rsp.status();

        let body_text = rsp.text().await?;

        if status != StatusCode::OK {
            let error_response = serde_json::from_str(&body_text).map_err(|e| {
                save_raw_json("log", "ErrorResponse", &body_text);
                Error::from(e)
            })?;

            return Err(Error::Response(error_response));
        }

        serde_json::from_str(&body_text).map_err(|e| {
            save_raw_json("log", "ExpirationChain", &body_text);
            Error::from(e)
        })
    }
}

/// Get `PriceHistory` for a single symbol and date ranges.
#[derive(Debug)]
pub struct GetPriceHistoryRequest {
    req: RequestBuilder,

    symbol: String,

    /// The chart period being requested.
    ///
    /// Available values : `day`, `month`, `year`, `ytd`
    period_type: Option<PeriodType>,

    /// The number of chart period types.
    ///
    /// If the [`Self::period_type`] is
    /// * `day` - valid values are `1`, `2`, `3`, `4`, `5`, `10`
    /// * `month` - valid values are `1`, `2`, `3`, `6`
    /// * `year` - valid values are `1`, `2`, `3`, `5`, `10`, `15`, `20`
    /// * `ytd` - valid values are `1`
    ///
    /// If the [`Self::period`] is not specified and the [`Self::period_type`] is
    /// * `day` - default period is `10`.
    /// * `month` - default period is `1`.
    /// * `year` - default period is `1`.
    /// * `ytd` - default period is `1`.
    period: Option<i64>,

    /// The time [`Self::frequency_type`]
    ///
    /// If the [`Self::period_type`] is
    /// * `day` - valid value is `minute`
    /// * `month` - valid values are `daily`, `weekly`
    /// * `year` - valid values are `daily`, `weekly`, `monthly`
    /// * `ytd` - valid values are `daily`, `weekly`
    ///
    /// If [`Self::frequency_type`] is not specified, default value depends on the [`Self::period_type`]
    /// * `day` - defaulted to `minute`.
    /// * `month` - defaulted to `weekly`.
    /// * `year` - defaulted to `monthly`.
    /// * `ytd` - defaulted to `weekly`.
    ///
    /// Available values : `minute`, `daily`, `weekly`, `monthly`
    frequency_type: Option<FrequencyType>,

    /// The time frequency duration
    ///
    /// If the [`Self::frequency_type`] is
    /// * `minute` - valid values are `1`, `5`, `10`, `15`, `30`
    /// * `daily` - valid value is `1`
    /// * `weekly` - valid value is `1`
    /// * `monthly` - valid value is `1`
    ///
    /// If [`Self::frequency`] is not specified, default value is `1`
    frequency: Option<i64>,

    // The start date, Time in milliseconds since the UNIX epoch eg 1451624400000
    /// If not specified [`Self::start_date`] will be ([`Self::end_date`] - [`Self::period`]) excluding weekends and holidays.
    start_date: Option<i64>,

    // The end date, Time in milliseconds since the UNIX epoch eg 1451624400000
    /// If not specified, the [`Self::end_date`] will default to the market close of previous business day.
    end_date: Option<i64>,

    /// Need extended hours data
    need_extended_hours_data: Option<bool>,

    /// Need previous close price/date
    need_previous_close: Option<bool>,
}

impl GetPriceHistoryRequest {
    fn endpoint() -> endpoints::EndpointPriceHistory {
        endpoints::EndpointPriceHistory::PriceHistory
    }

    pub(crate) fn new(client: &Client, access_token: String, symbol: String) -> Self {
        let req = client.get(Self::endpoint().url()).bearer_auth(access_token);
        Self::new_with(req, symbol)
    }

    fn new_with(req: RequestBuilder, symbol: String) -> Self {
        Self {
            req,
            symbol,
            period_type: None,
            period: None,
            frequency_type: None,
            frequency: None,
            start_date: None,
            end_date: None,
            need_extended_hours_data: None,
            need_previous_close: None,
        }
    }

    /// The chart period being requested.
    ///
    /// Available values : `day`, `month`, `year`, `ytd`
    pub fn period_type(&mut self, val: PeriodType) -> &mut Self {
        self.period_type = Some(val);
        self
    }

    /// The number of chart period types.
    ///
    /// If the [`Self::period_type`] is
    /// * day - valid values are 1, 2, 3, 4, 5, 10
    /// * month - valid values are 1, 2, 3, 6
    /// * year - valid values are 1, 2, 3, 5, 10, 15, 20
    /// * ytd - valid values are 1
    ///
    /// If the [`Self::period`] is not specified and the [`Self::period_type`] is
    /// * day - default period is 10.
    /// * month - default period is 1.
    /// * year - default period is 1.
    /// * ytd - default period is 1.
    pub fn period(&mut self, val: i64) -> &mut Self {
        self.period = Some(val);
        self
    }

    /// The time [`Self::frequency_type`]
    ///
    /// If the [`Self::period_type`] is
    /// * `day` - valid value is `minute`
    /// * `month` - valid values are `daily`, `weekly`
    /// * `year` - valid values are `daily`, `weekly`, `monthly`
    /// * `ytd` - valid values are `daily`, `weekly`
    ///
    /// If [`Self::frequency_type`] is not specified, default value depends on the [`Self::period_type`]
    /// * `day` - defaulted to `minute`.
    /// * `month` - defaulted to `weekly`.
    /// * `year` - defaulted to `monthly`.
    /// * `ytd` - defaulted to `weekly`.
    ///
    /// Available values : `minute`, `daily`, `weekly`, `monthly`
    pub fn frequency_type(&mut self, val: FrequencyType) -> &mut Self {
        self.frequency_type = Some(val);
        self
    }

    /// The time frequency duration
    ///
    /// If the [`Self::frequency_type`] is
    /// * `minute` - valid values are `1`, `5`, `10`, `15`, `30`
    /// * `daily` - valid value is `1`
    /// * `weekly` - valid value is `1`
    /// * `monthly` - valid value is `1`
    ///
    /// If [`Self::frequency`] is not specified, default value is `1`
    pub fn frequency(&mut self, val: i64) -> &mut Self {
        self.frequency = Some(val);
        self
    }

    /// If not specified [`Self::start_date`] will be ([`Self::end_date`] - [`Self::period`]) excluding weekends and holidays.
    pub fn start_date(&mut self, val: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.start_date = Some(val.timestamp_millis());
        self
    }

    /// If not specified, the [`Self::end_date`] will default to the market close of previous business day.
    pub fn end_date(&mut self, val: chrono::DateTime<chrono::Utc>) -> &mut Self {
        self.end_date = Some(val.timestamp_millis());
        self
    }

    /// Need extended hours data
    pub fn need_extended_hours_data(&mut self, val: bool) -> &mut Self {
        self.need_extended_hours_data = Some(val);
        self
    }

    /// Need previous close price/date
    pub fn need_previous_close(&mut self, val: bool) -> &mut Self {
        self.need_previous_close = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req.query(&[("symbol", self.symbol)]);
        if let Some(x) = self.period_type {
            req = req.query(&[("periodType", x)]);
        }
        if let Some(x) = self.period {
            req = req.query(&[("period", x)]);
        }
        if let Some(x) = self.frequency_type {
            req = req.query(&[("frequencyType", x)]);
        }
        if let Some(x) = self.frequency {
            req = req.query(&[("frequency", x)]);
        }
        if let Some(x) = self.start_date {
            req = req.query(&[("startDate", x)]);
        }
        if let Some(x) = self.end_date {
            req = req.query(&[("endDate", x)]);
        }
        if let Some(x) = self.need_extended_hours_data {
            req = req.query(&[("needExtendedHoursData", x)]);
        }
        if let Some(x) = self.need_previous_close {
            req = req.query(&[("needPreviousClose", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<model::CandleList, Error> {
        let req = self.build();
        let rsp = req.send().await?;
        let status = rsp.status();

        let body_text = rsp.text().await?;

        if status != StatusCode::OK {
            let error_response = serde_json::from_str(&body_text).map_err(|e| {
                save_raw_json("log", "ErrorResponse", &body_text);
                Error::from(e)
            })?;

            return Err(Error::Response(error_response));
        }

        serde_json::from_str(&body_text).map_err(|e| {
            save_raw_json("log", "CandleList", &body_text);
            Error::from(e)
        })
    }
}

/// Get Movers for a specific index.
#[derive(Debug)]
pub struct GetMoversRequest {
    req: RequestBuilder,

    /// Index Symbol
    ///
    /// Available values : `$DJI`, `$COMPX`, `$SPX`, `NYSE`, `NASDAQ`, `OTCBB`, `INDEX_ALL`, `EQUITY_ALL`, `OPTION_ALL`, `OPTION_PUT`, `OPTION_CALL`
    ///
    /// Example : `$DJI`
    symbol: String,

    /// Sort by a particular attribute
    ///
    /// Available values : `VOLUME`, `TRADES`, `PERCENT_CHANGE_UP`, `PERCENT_CHANGE_DOWN`
    ///
    /// Example : `VOLUME`
    sort: Option<SortAttribute>,

    /// To return movers with the specified directions of up or down
    ///
    /// Available values : `0`, `1`, `5`, `10`, `30`, `60`
    ///
    /// Default value : `0`
    frequency: Option<i64>,
}

impl GetMoversRequest {
    fn endpoint(symbol_id: String) -> endpoints::EndpointMover {
        endpoints::EndpointMover::Mover { symbol_id }
    }

    pub(crate) fn new(client: &Client, access_token: String, symbol: String) -> Self {
        let req = client
            .get(Self::endpoint(symbol.clone()).url())
            .bearer_auth(access_token);

        Self::new_with(req, symbol)
    }

    fn new_with(req: RequestBuilder, symbol: String) -> Self {
        Self {
            req,
            symbol,
            sort: None,
            frequency: None,
        }
    }

    /// Sort by a particular attribute
    ///
    /// Available values : `VOLUME`, `TRADES`, `PERCENT_CHANGE_UP`, `PERCENT_CHANGE_DOWN`
    ///
    /// Example : `VOLUME`
    pub fn sort(&mut self, val: SortAttribute) -> &mut Self {
        self.sort = Some(val);
        self
    }

    /// To return movers with the specified directions of up or down
    ///
    /// Available values : `0`, `1`, `5`, `10`, `30`, `60`
    ///
    /// Default value : `0`
    pub fn frequency(&mut self, val: i64) -> &mut Self {
        self.frequency = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req.query(&[("symbol", self.symbol)]);
        if let Some(x) = self.sort {
            req = req.query(&[("sort", x)]);
        }
        if let Some(x) = self.frequency {
            req = req.query(&[("frequency", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<model::Mover, Error> {
        let req = self.build();
        let rsp = req.send().await?;
        let status = rsp.status();

        let body_text = rsp.text().await?;

        if status != StatusCode::OK {
            let error_response = serde_json::from_str(&body_text).map_err(|e| {
                save_raw_json("log", "ErrorResponse", &body_text);
                Error::from(e)
            })?;

            return Err(Error::Response(error_response));
        }

        serde_json::from_str(&body_text).map_err(|e| {
            save_raw_json("log", "Mover", &body_text);
            Error::from(e)
        })
    }
}

/// Get Market Hours for different markets.
#[derive(Debug)]
pub struct GetMarketsRequest {
    req: RequestBuilder,

    /// List of markets
    ///
    /// Available values : `equity`, `option`, `bond`, `future`, `forex`
    markets: Vec<Market>,

    /// Valid date range is from currentdate to 1 year from today.
    ///
    /// It will default to current day if not entered.
    // Date format:YYYY-MM-DD
    date: Option<chrono::NaiveDate>,
}

impl GetMarketsRequest {
    fn endpoint() -> endpoints::EndpointMarketHour {
        endpoints::EndpointMarketHour::Markets
    }

    pub(crate) fn new(client: &Client, access_token: String, markets: Vec<Market>) -> Self {
        let req = client.get(Self::endpoint().url()).bearer_auth(access_token);

        Self::new_with(req, markets)
    }

    fn new_with(req: RequestBuilder, markets: Vec<Market>) -> Self {
        Self {
            req,
            markets,
            date: None,
        }
    }

    /// Valid date range is from currentdate to 1 year from today.
    ///
    /// It will default to current day if not entered.
    pub fn date(&mut self, val: chrono::NaiveDate) -> &mut Self {
        self.date = Some(val);
        self
    }

    #[tracing::instrument(skip(self), fields(market_count = self.markets.len()))]
    fn build(self) -> RequestBuilder {
        tracing::debug!("building market hours request");

        let market_strs = self.markets.iter().map(|m| m.as_str()).collect::<Vec<_>>();
        let markets_param = market_strs.join(",");

        tracing::debug!(markets = %markets_param, "markets parameter constructed");

        let mut req = self.req.query(&[("markets", markets_param)]);

        if let Some(date) = self.date {
            tracing::debug!(%date, "applying date filter to market hours request");
            req = req.query(&[("date", date)]);
        }

        tracing::debug!("market hours request builder finalized");

        req
    }

    pub async fn send(self) -> Result<model::Markets, Error> {
        let req = self.build();
        let rsp = req.send().await?;
        let status = rsp.status();

        let body_text = rsp.text().await?;

        if status != StatusCode::OK {
            let error_response = serde_json::from_str(&body_text).map_err(|e| {
                save_raw_json("log", "ErrorResponse", &body_text);
                Error::from(e)
            })?;

            return Err(Error::Response(error_response));
        }

        serde_json::from_str(&body_text).map_err(|e| {
            save_raw_json("log", "Markets", &body_text);
            Error::from(e)
        })
    }
}

/// Get Market Hours for a single market.
#[derive(Debug)]
pub struct GetMarketRequest {
    req: RequestBuilder,

    /// Available values : `equity`, `option`, `bond`, `future`, `forex`
    market_id: Market,

    /// Valid date range is from currentdate to 1 year from today.
    ///
    /// It will default to current day if not entered.
    // Date format:YYYY-MM-DD
    date: Option<chrono::NaiveDate>,
}

impl GetMarketRequest {
    fn endpoint(market_id: Market) -> endpoints::EndpointMarketHour {
        endpoints::EndpointMarketHour::Market { market_id }
    }

    pub(crate) fn new(client: &Client, access_token: String, market_id: Market) -> Self {
        let req = client
            .get(Self::endpoint(market_id).url())
            .bearer_auth(access_token);

        Self::new_with(req, market_id)
    }

    fn new_with(req: RequestBuilder, market_id: Market) -> Self {
        Self {
            req,
            market_id,
            date: None,
        }
    }

    /// Valid date range is from currentdate to 1 year from today.
    ///
    /// It will default to current day if not entered.
    pub fn date(&mut self, val: chrono::NaiveDate) -> &mut Self {
        self.date = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req.query(&[("market_id", self.market_id)]);
        if let Some(x) = self.date {
            req = req.query(&[("date", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<model::Markets, Error> {
        let req = self.build();
        let rsp = req.send().await?;
        let status = rsp.status();

        let body_text = rsp.text().await?;

        if status != StatusCode::OK {
            let error_response = serde_json::from_str(&body_text).map_err(|e| {
                save_raw_json("log", "ErrorResponse", &body_text);
                Error::from(e)
            })?;

            return Err(Error::Response(error_response));
        }

        serde_json::from_str(&body_text).map_err(|e| {
            save_raw_json("log", "Markets", &body_text);
            Error::from(e)
        })
    }
}

/// Get Instruments by symbols and projections.
#[derive(Debug)]
pub struct GetInstrumentsRequest {
    req: RequestBuilder,

    symbol: String,

    /// search by
    ///
    /// Available values : `symbol-search`, `symbol-regex`, `desc-search`, `desc-regex`, `search`, `fundamental`
    projection: Projection,
}

impl GetInstrumentsRequest {
    fn endpoint() -> endpoints::EndpointInstrument {
        endpoints::EndpointInstrument::Instruments
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        symbol: String,
        projection: Projection,
    ) -> Self {
        let req = client.get(Self::endpoint().url()).bearer_auth(access_token);
        Self::new_with(req, symbol, projection)
    }

    fn new_with(req: RequestBuilder, symbol: String, projection: Projection) -> Self {
        Self {
            req,
            symbol,
            projection,
        }
    }

    fn build(self) -> RequestBuilder {
        self.req
            .query(&[("symbol", self.symbol)])
            .query(&[("projection", self.projection)])
    }

    pub async fn send(self) -> Result<model::Instruments, Error> {
        let req = self.build();
        let rsp = req.send().await?;
        let status = rsp.status();

        let body_text = rsp.text().await?;

        if status != StatusCode::OK {
            let error_response = serde_json::from_str(&body_text).map_err(|e| {
                save_raw_json("log", "ErrorResponse", &body_text);
                Error::from(e)
            })?;

            return Err(Error::Response(error_response));
        }

        serde_json::from_str(&body_text).map_err(|e| {
            save_raw_json("log", "Instruments", &body_text);
            Error::from(e)
        })
    }
}

/// Get Instrument by specific cusip
#[derive(Debug)]
pub struct GetInstrumentRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    /// cusip of a security
    cusip_id: String,
}

impl GetInstrumentRequest {
    fn endpoint(cusip_id: String) -> endpoints::EndpointInstrument {
        endpoints::EndpointInstrument::Instrutment { cusip_id }
    }

    pub(crate) fn new(client: &Client, access_token: String, cusip_id: String) -> Self {
        let req = client
            .get(Self::endpoint(cusip_id.clone()).url())
            .bearer_auth(access_token);
        Self::new_with(req, cusip_id)
    }

    fn new_with(req: RequestBuilder, cusip_id: String) -> Self {
        Self { req, cusip_id }
    }

    fn build(self) -> RequestBuilder {
        self.req
    }

    pub async fn send(self) -> Result<model::InstrumentResponse, Error> {
        let cusip_id = self.cusip_id.clone();
        let req = self.build();

        let rsp = req.send().await.map_err(|e| {
            tracing::error!(error = %e, "network request failed");
            e
        })?;

        let status = rsp.status();
        let body_text = rsp.text().await.map_err(|e| {
            tracing::error!(error = %e, "failed to read response body");
            e
        })?;

        if status != StatusCode::OK {
            tracing::warn!(%status, "received non-OK response from server");

            let error_response = serde_json::from_str(&body_text).map_err(|e| {
                save_raw_json("log", "ErrorResponse", &body_text);
                e
            })?;

            return Err(Error::Response(error_response));
        }

        let mut data: model::Instruments = serde_json::from_str(&body_text).map_err(|e| {
            save_raw_json("log", "Instruments", &body_text);
            tracing::error!(error = %e, "failed to parse Instruments JSON");
            e
        })?;

        let count = data.instruments.len();
        let instrument = if data.instruments.is_empty() {
            // 記錄警告，並標註具體的 cusip_id
            tracing::warn!(cusip = %cusip_id, "no instruments found for the given CUSIP");

            return Err(Error::Quote(model::QuoteError {
                invalid_symbols: None,
                invalid_cusips: Some(vec![cusip_id]), // 直接將當前的 ID 封裝進錯誤
                invalid_ssids: None,
            }));
        } else {
            // 取出最匹配的一筆
            data.instruments.remove(0)
        };

        tracing::info!(
            remaining_results = %(count - 1),
            cusip = %cusip_id,
            "instrument retrieved successfully"
        );

        Ok(instrument)
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;
    use pretty_assertions::assert_eq;
    use reqwest::Client;
    use test_log::test;

    use super::*;

    #[test(tokio::test)]
    async fn test_get_quotes_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let symbols = vec!["symbol1".to_string(), "symbol2".to_string()];
        let fields = vec![
            QuoteField::Reference,
            QuoteField::Regular,
            QuoteField::Extra("Extra".to_string()),
        ];
        let indicative = true;

        // Create a mock
        let mock = server
            .mock("GET", "/quotes")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbols".into(), symbols.join(",")),
                Matcher::UrlEncoded("fields".into(), "reference,regular,Extra".into()),
                Matcher::UrlEncoded("indicative".into(), indicative.to_string()),
            ]))
            // .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/MarketData/QuoteResponse.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetQuotesRequest::endpoint().url_endpoint()
        ));

        let mut req = GetQuotesRequest::new_with(req, symbols.clone());

        // check initial value
        assert_eq!(req.symbols, symbols);
        assert_eq!(req.fields, None);
        assert_eq!(req.indicative, None);

        // check setter
        req.fields(fields.clone());
        assert_eq!(req.fields, Some(fields));
        req.indicative(indicative);
        assert_eq!(req.indicative, Some(indicative));

        tracing::debug!(?req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.len(), 17);
    }

    #[test(tokio::test)]
    async fn test_get_quotes_request_real() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let symbols = vec!["symbol1".to_string(), "symbol2".to_string()];
        let fields = vec![
            QuoteField::Reference,
            QuoteField::Regular,
            QuoteField::Extra("Extra".to_string()),
        ];
        let indicative = true;

        // Create a mock
        let mock = server
            .mock("GET", "/quotes")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbols".into(), symbols.join(",")),
                Matcher::UrlEncoded("fields".into(), "reference,regular,Extra".into()),
                Matcher::UrlEncoded("indicative".into(), indicative.to_string()),
            ]))
            // .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/MarketData/QuoteResponse_real.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetQuotesRequest::endpoint().url_endpoint()
        ));

        let mut req = GetQuotesRequest::new_with(req, symbols.clone());

        // check initial value
        assert_eq!(req.symbols, symbols);
        assert_eq!(req.fields, None);
        assert_eq!(req.indicative, None);

        // check setter
        req.fields(fields.clone());
        assert_eq!(req.fields, Some(fields));
        req.indicative(indicative);
        assert_eq!(req.indicative, Some(indicative));

        tracing::debug!(?req);
        let result = req.send().await;
        mock.assert_async().await;
        result.unwrap();
    }

    #[test(tokio::test)]
    #[allow(clippy::too_many_lines)]
    async fn test_get_quote_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let symbol = "AAPL".to_string();
        let fields = vec![QuoteField::Reference, QuoteField::Regular];

        // Create a mock
        let mock = server
            .mock("GET", "/AAPL/quotes")
            .match_query(Matcher::UrlEncoded(
                "fields".into(),
                "reference,regular".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
						"AAPL": {
							"assetMainType": "EQUITY",
							"assetSubType": "COE",
							"quoteType": "NBBO",
							"realtime": true,
							"ssid": 1973757747,
							"symbol": "AAPL",
							"fundamental": {
								"avg10DaysVolume": 74260136,
								"avg1YearVolume": 58373005,
								"declarationDate": "2024-05-02T04:00:00Z",
								"divAmount": 1,
								"divExDate": "2024-05-10T04:00:00Z",
								"divFreq": 4,
								"divPayAmount": 0.25,
								"divPayDate": "2024-05-16T04:00:00Z",
								"divYield": 0.5463,
								"eps": 6.13,
								"fundLeverageFactor": 0,
								"lastEarningsDate": "2024-05-02T04:00:00Z",
								"nextDivExDate": "2024-08-12T04:00:00Z",
								"nextDivPayDate": "2024-08-16T04:00:00Z",
								"peRatio": 28.51406
							},
							"quote": {
								"52WeekHigh": 199.62,
								"52WeekLow": 164.075,
								"askMICId": "EDGX",
								"askPrice": 184.98,
								"askSize": 3,
								"askTime": 1715594417785,
								"bidMICId": "EDGX",
								"bidPrice": 184.91,
								"bidSize": 1,
								"bidTime": 1715594417785,
								"closePrice": 183.05,
								"highPrice": 0,
								"lastMICId": "ARCX",
								"lastPrice": 184.92,
								"lastSize": 9,
								"lowPrice": 0,
								"mark": 184.91,
								"markChange": 1.86,
								"markPercentChange": 1.01611582,
								"netChange": 1.87,
								"netPercentChange": 1.0215788,
								"openPrice": 0,
								"postMarketChange": 1.87,
								"postMarketPercentChange": 1.0215788,
								"quoteTime": 1715594417785,
								"securityStatus": "Normal",
								"totalVolume": 138478,
								"tradeTime": 1715594427508
							},
							"reference": {
								"cusip": "037833100",
								"description": "Apple Inc",
								"exchange": "Q",
								"exchangeName": "NASDAQ",
								"isHardToBorrow": false,
								"isShortable": true,
								"htbRate": 0
							},
							"regular": {
								"regularMarketLastPrice": 183.05,
								"regularMarketLastSize": 7250871,
								"regularMarketNetChange": 0,
								"regularMarketPercentChange": 0,
								"regularMarketTradeTime": 1715371200231
							}
						}
					}"#,
            )
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetQuoteRequest::endpoint(symbol.clone()).url_endpoint()
        ));
        let mut req = GetQuoteRequest::new_with(req, symbol.clone());

        // check initial value
        assert_eq!(req.symbol, symbol);
        assert_eq!(req.fields, None);

        // check setter
        req.fields(fields.clone());
        assert_eq!(req.fields, Some(fields));

        tracing::debug!(?req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        match result {
            model::QuoteResponse::Equity(x) => assert_eq!(x.symbol, symbol),
            x => panic!("{x:?} is not Equity"),
        }
    }

    #[test(tokio::test)]
    async fn test_get_quote_request_error() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let symbol = "^IRX".to_string();
        let fields = vec![QuoteField::Reference, QuoteField::Regular];

        // Create a mock
        let mock = server
            .mock("GET", "/%5EIRX/quotes")
            .match_query(Matcher::UrlEncoded(
                "fields".into(),
                "reference,regular".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
					"errors": {
						"invalidSymbols": [
							"^IRX"
						]
					}
				}"#,
            )
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetQuoteRequest::endpoint(symbol.clone()).url_endpoint()
        ));
        let mut req = GetQuoteRequest::new_with(req, symbol.clone());

        // check initial value
        assert_eq!(req.symbol, symbol);
        assert_eq!(req.fields, None);

        // check setter
        req.fields(fields.clone());
        assert_eq!(req.fields, Some(fields));

        tracing::debug!(?req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap_err();
        match result {
            Error::Quote(model::QuoteError {
                invalid_symbols: Some(e),
                ..
            }) => assert_eq!(e, vec!["^IRX"]),
            x => panic!("{x:?} is not QuoteError"),
        }
    }

    #[allow(clippy::too_many_lines)]
    #[test(tokio::test)]
    async fn test_get_options_chains_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let symbol = "string".to_string();
        let contract_type = ContractType::Call;
        let strike_count = 1;
        let include_underlying_quote = true;
        let strategy = OptionChainStrategy::Single;
        let interval = 1.1;
        let strike = 2.2;
        let range = "ITM".to_string();
        let from_date = chrono::NaiveDate::from_ymd_opt(2015, 3, 14).unwrap();
        let to_date = chrono::NaiveDate::from_ymd_opt(2015, 5, 14).unwrap();
        let volatility = 3.3;
        let underlying_price = 4.4;
        let interest_rate = 5.5;
        let days_to_expiration = 2;
        let exp_month = Month::Jan;
        let option_type = "option_type".to_string();
        let entitlement = Entitlement::PN;

        // Create a mock
        let mock = server
            .mock("GET", "/chains")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbol".into(), symbol.clone()),
                Matcher::UrlEncoded("contractType".into(), "CALL".into()),
                Matcher::UrlEncoded("strikeCount".into(), strike_count.to_string()),
                Matcher::UrlEncoded(
                    "includeUnderlyingQuote".into(),
                    include_underlying_quote.to_string(),
                ),
                Matcher::UrlEncoded("strategy".into(), "SINGLE".into()),
                Matcher::UrlEncoded("interval".into(), interval.to_string()),
                Matcher::UrlEncoded("strike".into(), strike.to_string()),
                Matcher::UrlEncoded("range".into(), range.clone()),
                Matcher::UrlEncoded("fromDate".into(), from_date.to_string()),
                Matcher::UrlEncoded("toDate".into(), to_date.to_string()),
                Matcher::UrlEncoded("volatility".into(), volatility.to_string()),
                Matcher::UrlEncoded("underlyingPrice".into(), underlying_price.to_string()),
                Matcher::UrlEncoded("interestRate".into(), interest_rate.to_string()),
                Matcher::UrlEncoded("daysToExpiration".into(), days_to_expiration.to_string()),
                Matcher::UrlEncoded("expMonth".into(), "JAN".into()),
                Matcher::UrlEncoded("optionType".into(), option_type.clone()),
                Matcher::UrlEncoded("entitlement".into(), "PN".into()),
            ]))
            // .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/MarketData/OptionChain_real.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetOptionChainsRequest::endpoint().url_endpoint()
        ));
        let mut req = GetOptionChainsRequest::new_with(req, symbol.clone());

        // check initial value
        assert_eq!(req.symbol, symbol);
        assert_eq!(req.contract_type, None);
        assert_eq!(req.strike_count, None);
        assert_eq!(req.include_underlying_quote, None);
        assert_eq!(req.strategy, None);
        assert_eq!(req.interval, None);
        assert_eq!(req.strike, None);
        assert_eq!(req.range, None);
        assert_eq!(req.from_date, None);
        assert_eq!(req.to_date, None);
        assert_eq!(req.volatility, None);
        assert_eq!(req.underlying_price, None);
        assert_eq!(req.interest_rate, None);
        assert_eq!(req.days_to_expiration, None);
        assert_eq!(req.exp_month, None);
        assert_eq!(req.option_type, None);
        assert_eq!(req.entitlement, None);

        // check setter
        req.contract_type(contract_type);
        assert_eq!(req.contract_type, Some(contract_type));
        req.strike_count(strike_count);
        assert_eq!(req.strike_count, Some(strike_count));
        req.include_underlying_quote(include_underlying_quote);
        assert_eq!(req.include_underlying_quote, Some(include_underlying_quote));
        req.strategy(strategy);
        assert_eq!(req.strategy, Some(strategy));
        req.interval(interval);
        assert_eq!(req.interval, Some(interval));
        req.strike(strike);
        assert_eq!(req.strike, Some(strike));
        req.range(range.clone());
        assert_eq!(req.range, Some(range));
        req.from_date(from_date);
        assert_eq!(req.from_date, Some(from_date));
        req.to_date(to_date);
        assert_eq!(req.to_date, Some(to_date));
        req.volatility(volatility);
        assert_eq!(req.volatility, Some(volatility));
        req.underlying_price(underlying_price);
        assert_eq!(req.underlying_price, Some(underlying_price));
        req.interest_rate(interest_rate);
        assert_eq!(req.interest_rate, Some(interest_rate));
        req.days_to_expiration(days_to_expiration);
        assert_eq!(req.days_to_expiration, Some(days_to_expiration));
        req.exp_month(exp_month);
        assert_eq!(req.exp_month, Some(exp_month));
        req.option_type(option_type.clone());
        assert_eq!(req.option_type, Some(option_type));
        req.entitlement(entitlement);
        assert_eq!(req.entitlement, Some(entitlement));

        tracing::debug!(?req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.status, "SUCCESS");
    }

    #[test(tokio::test)]
    async fn test_get_option_expiration_chain_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let symbol = "string".to_string();

        // Create a mock
        let mock = server
            .mock("GET", "/expirationchain")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "symbol".into(),
                symbol.clone(),
            )]))
            // .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/MarketData/ExpirationChain_real.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetOptionExpirationChainRequest::endpoint().url_endpoint()
        ));
        let req = GetOptionExpirationChainRequest::new_with(req, symbol.clone());

        // check initial value
        assert_eq!(req.symbol, symbol);

        // check setter
        // none

        tracing::debug!(?req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.expiration_list.len(), 21);
    }

    #[test(tokio::test)]
    async fn test_get_price_history_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let symbol = "AAPL".to_string();
        let period_type = PeriodType::Day;
        let period = 1;
        let frequency_type = FrequencyType::Minute;
        let frequency = 2;
        let start_date = chrono::NaiveDate::from_ymd_opt(2015, 1, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 1, 444)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap();
        let end_date = chrono::NaiveDate::from_ymd_opt(2016, 1, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 1, 444)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap();
        let need_extended_hours_data = true;
        let need_previous_close = false;

        // Create a mock
        let mock = server
            .mock("GET", "/pricehistory")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbol".into(), symbol.clone()),
                Matcher::UrlEncoded("periodType".into(), "day".into()),
                Matcher::UrlEncoded("period".into(), period.to_string()),
                Matcher::UrlEncoded("frequencyType".into(), "minute".into()),
                Matcher::UrlEncoded("frequency".into(), frequency.to_string()),
                Matcher::UrlEncoded(
                    "startDate".into(),
                    start_date.timestamp_millis().to_string(),
                ),
                Matcher::UrlEncoded("endDate".into(), end_date.timestamp_millis().to_string()),
                Matcher::UrlEncoded(
                    "needExtendedHoursData".into(),
                    need_extended_hours_data.to_string(),
                ),
                Matcher::UrlEncoded("needPreviousClose".into(), need_previous_close.to_string()),
            ]))
            // .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/MarketData/CandleList.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetPriceHistoryRequest::endpoint().url_endpoint()
        ));
        let mut req = GetPriceHistoryRequest::new_with(req, symbol.clone());

        // check initial value
        assert_eq!(req.symbol, symbol);
        assert_eq!(req.period_type, None);
        assert_eq!(req.period, None);
        assert_eq!(req.frequency_type, None);
        assert_eq!(req.frequency, None);
        assert_eq!(req.start_date, None);
        assert_eq!(req.end_date, None);
        assert_eq!(req.need_extended_hours_data, None);
        assert_eq!(req.need_previous_close, None);

        // check setter
        req.period_type(period_type);
        assert_eq!(req.period_type, Some(period_type));
        req.period(period);
        assert_eq!(req.period, Some(period));
        req.frequency_type(frequency_type);
        assert_eq!(req.frequency_type, Some(frequency_type));
        req.frequency(frequency);
        assert_eq!(req.frequency, Some(frequency));
        req.start_date(start_date);
        assert_eq!(req.start_date, Some(start_date.timestamp_millis()));
        req.end_date(end_date);
        assert_eq!(req.end_date, Some(end_date.timestamp_millis()));
        req.need_extended_hours_data(need_extended_hours_data);
        assert_eq!(req.need_extended_hours_data, Some(need_extended_hours_data));
        req.need_previous_close(need_previous_close);
        assert_eq!(req.need_previous_close, Some(need_previous_close));

        tracing::debug!(?req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.symbol, "AAPL");
    }

    #[test(tokio::test)]
    async fn test_get_movers_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let symbol = "$DJI".to_string();
        let sort = SortAttribute::Volume;
        let frequency = 1;

        // Create a mock
        let mock = server
            .mock("GET", "/movers/%24DJI")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("sort".into(), "VOLUME".into()),
                Matcher::UrlEncoded("frequency".into(), frequency.to_string()),
            ]))
            // .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/MarketData/Mover.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetMoversRequest::endpoint(symbol.clone()).url_endpoint()
        ));
        let mut req = GetMoversRequest::new_with(req, symbol.clone());

        // check initial value
        assert_eq!(req.symbol, symbol);
        assert_eq!(req.sort, None);
        assert_eq!(req.frequency, None);

        // check setter
        req.sort(sort);
        assert_eq!(req.sort, Some(sort));
        req.frequency(frequency);
        assert_eq!(req.frequency, Some(frequency));

        tracing::debug!(?req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.screeners.len(), 3);
    }

    #[test(tokio::test)]
    async fn test_get_markets_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let markets = vec![Market::Equity, Market::Option];
        let date = chrono::NaiveDate::from_ymd_opt(2015, 3, 14).unwrap();

        // Create a mock
        let mock = server
            .mock("GET", "/markets")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("markets".into(), "equity,option".into()),
                Matcher::UrlEncoded("date".into(), date.to_string()),
            ]))
            // .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/MarketData/Markets.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetMarketsRequest::endpoint().url_endpoint()
        ));
        let mut req = GetMarketsRequest::new_with(req, markets.clone());

        // check initial value
        assert_eq!(req.markets, markets);
        assert_eq!(req.date, None);

        // check setter
        req.date(date);
        assert_eq!(req.date, Some(date));

        tracing::debug!(?req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test(tokio::test)]
    async fn test_get_market_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let market_id = Market::Equity;
        let date = chrono::NaiveDate::from_ymd_opt(2015, 3, 14).unwrap();

        // Create a mock
        let mock = server
            .mock("GET", "/markets/equity")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "date".into(),
                date.to_string(),
            )]))
            // .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
					"equity": {
						"EQ": {
							"date": "2022-04-14",
							"marketType": "EQUITY",
							"exchange": "NULL",
							"category": "NULL",
							"product": "EQ",
							"productName": "equity",
							"isOpen": true,
							"sessionHours": {
								"preMarket": [
									{
										"start": "2022-04-14T07:00:00-04:00",
										"end": "2022-04-14T09:30:00-04:00"
									}
								],
								"regularMarket": [
									{
										"start": "2022-04-14T09:30:00-04:00",
										"end": "2022-04-14T16:00:00-04:00"
									}
								],
								"postMarket": [
									{
										"start": "2022-04-14T16:00:00-04:00",
										"end": "2022-04-14T20:00:00-04:00"
									}
								]
							}
						}
					}
				}"#,
            )
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetMarketRequest::endpoint(market_id).url_endpoint()
        ));
        let mut req = GetMarketRequest::new_with(req, market_id);

        // check initial value
        assert_eq!(req.market_id, market_id);
        assert_eq!(req.date, None);

        // check setter
        req.date(date);
        assert_eq!(req.date, Some(date));

        tracing::debug!(?req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.keys().next().unwrap(), "equity");
    }

    #[test(tokio::test)]
    async fn test_get_instruments_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let symbol = "AAPL".to_string();
        let projection = Projection::SymbolSearch;

        // Create a mock
        let mock = server
            .mock("GET", "/instruments")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbol".into(), symbol.clone()),
                Matcher::UrlEncoded("projection".into(), "symbol-search".into()),
            ]))
            // .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/MarketData/Instruments.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetInstrumentsRequest::endpoint().url_endpoint()
        ));
        let req = GetInstrumentsRequest::new_with(req, symbol.clone(), projection);

        // check initial value
        assert_eq!(req.symbol, symbol);
        assert_eq!(req.projection, projection);

        // check setter
        // none

        tracing::debug!(?req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.instruments.len(), 2);
    }

    #[test(tokio::test)]
    async fn test_get_instrument_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let cusip_id = "037833100".to_string();

        // Create a mock
        let mock = server
            .mock("GET", "/instruments/037833100")
            // .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
					"instruments": [
						{
							"cusip": "922908769",
							"symbol": "VTI",
							"description": "VANGUARD TOTAL STOCK MARKET ETF",
							"exchange": "NYSE Arca",
							"assetType": "ETF"
						}
					]
				}"#,
            )
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetInstrumentRequest::endpoint(cusip_id.clone()).url_endpoint()
        ));
        let req = GetInstrumentRequest::new_with(req, cusip_id.clone());

        // check initial value
        assert_eq!(req.cusip_id, cusip_id);

        // check setter
        // none

        tracing::debug!(?req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.cusip, "922908769");
    }
}
