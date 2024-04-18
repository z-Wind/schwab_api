use reqwest::{RequestBuilder, StatusCode};
use std::collections::HashMap;

use crate::api::Error;
use crate::model;

#[derive(Debug)]
pub struct GetQuotesRequest {
    req: RequestBuilder,

    symbols: Vec<String>,

    // Request for subset of data by passing coma separated list of root nodes,
    // possible root nodes are quote, fundamental, extended, reference, regular. Sending quote, fundamental in request will return quote and fundamental data in response.
    // Dont send this attribute for full response.
    // Default value : all
    fields: Vec<String>,

    // Include indicative symbol quotes for all ETF symbols in request.
    // If ETF symbol ABC is in request and indicative=true API will return quotes for ABC and its corresponding indicative quote for $ABC.IV
    indicative: Option<bool>,
}

impl GetQuotesRequest {
    pub(crate) fn new(req: RequestBuilder, symbols: Vec<String>) -> Self {
        Self {
            req,
            symbols,
            fields: vec!["all".to_string()],
            indicative: None,
        }
    }

    pub fn fields(mut self, val: Vec<String>) -> Self {
        self.fields = val;
        self
    }

    pub fn indicative(mut self, val: bool) -> Self {
        self.indicative = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req.query(&[
            ("symbols", self.symbols.join(",")),
            ("fields", self.fields.join(",")),
        ]);
        if let Some(x) = self.indicative {
            req = req.query(&[("indicative", x.to_string())]);
        }

        req
    }

    pub async fn send(self) -> Result<HashMap<String, model::QuoteResponse>, Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ErrorResponse>().await?;
            return Err(Error::ErrorResponse(error_response));
        }

        rsp.json::<HashMap<String, model::QuoteResponse>>()
            .await
            .map_err(std::convert::Into::into)
    }
}

#[derive(Debug)]
pub struct GetQuoteRequest {
    req: RequestBuilder,

    symbol: String,

    // Request for subset of data by passing coma separated list of root nodes,
    // possible root nodes are quote, fundamental, extended, reference, regular. Sending quote, fundamental in request will return quote and fundamental data in response.
    // Dont send this attribute for full response.
    // Default value : all
    fields: Vec<String>,
}

impl GetQuoteRequest {
    pub(crate) fn new(req: RequestBuilder, symbol: String) -> Self {
        Self {
            req,
            symbol,
            fields: vec!["all".to_string()],
        }
    }

    pub fn fields(mut self, val: Vec<String>) -> Self {
        self.fields = val;
        self
    }

    fn build(self) -> RequestBuilder {
        let req = self.req.query(&[("fields", self.fields.join(","))]);

        req
    }

    pub async fn send(self) -> Result<model::QuoteResponse, Error> {
        let symbol = self.symbol.clone();
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ErrorResponse>().await?;
            return Err(Error::ErrorResponse(error_response));
        }

        let mut map = rsp.json::<HashMap<String, model::QuoteResponse>>().await?;
        Ok(map.remove(&symbol).expect("must exist"))
    }
}

#[derive(Debug)]
pub struct GetOptionChainsRequest {
    req: RequestBuilder,

    symbol: String,

    // Contract Type
    // Available values : CALL, PUT, ALL
    contract_type: Option<String>,
    // The Number of strikes to return above or below the at-the-money price
    strike_count: Option<i64>,
    // Underlying quotes to be included
    include_underlying_quote: Option<bool>,
    // OptionChain strategy.
    // Default is SINGLE.
    // ANALYTICAL allows the use of volatility, underlyingPrice, interestRate, and daysToExpiration params to calculate theoretical values.
    // Available values : SINGLE, ANALYTICAL, COVERED, VERTICAL, CALENDAR, STRANGLE, STRADDLE, BUTTERFLY, CONDOR, DIAGONAL, COLLAR, ROLL
    strategy: String,
    // Strike interval for spread strategy chains (see strategy param)
    interval: Option<f64>,
    // Strike Price
    strike: Option<f64>,
    // Range(ITM/NTM/OTM etc.)
    range: Option<String>,
    // From date(pattern: yyyy-MM-dd)
    from_date: Option<chrono::NaiveDate>,
    // To date (pattern: yyyy-MM-dd)
    to_date: Option<chrono::NaiveDate>,
    // Volatility to use in calculations. Applies only to ANALYTICAL strategy chains (see strategy param)
    volatility: Option<f64>,
    // Underlying price to use in calculations. Applies only to ANALYTICAL strategy chains (see strategy param)
    underlying_price: Option<f64>,
    // Interest rate to use in calculations. Applies only to ANALYTICAL strategy chains (see strategy param)
    interest_rate: Option<f64>,
    // Days to expiration to use in calculations. Applies only to ANALYTICAL strategy chains (see strategy param)
    days_to_expiration: Option<i64>,
    // Expiration month
    // Available values : JAN, FEB, MAR, APR, MAY, JUN, JUL, AUG, SEP, OCT, NOV, DEC, ALL
    exp_month: Option<String>,
    // Option Type
    option_type: Option<String>,
    // Applicable only if its retail token, entitlement of client PP-PayingPro, NP-NonPro and PN-NonPayingPro
    // Available values : PN, NP, PP
    entitlement: Option<String>,
}

impl GetOptionChainsRequest {
    pub(crate) fn new(req: RequestBuilder, symbol: String) -> Self {
        Self {
            req,
            symbol,
            contract_type: None,
            strike_count: None,
            include_underlying_quote: None,
            strategy: "SINGLE".to_string(),
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

    pub fn contract_type(mut self, val: String) -> Self {
        self.contract_type = Some(val);
        self
    }

    pub fn strike_count(mut self, val: i64) -> Self {
        self.strike_count = Some(val);
        self
    }

    pub fn include_underlying_quote(mut self, val: bool) -> Self {
        self.include_underlying_quote = Some(val);
        self
    }

    pub fn strategy(mut self, val: String) -> Self {
        self.strategy = val;
        self
    }

    pub fn interval(mut self, val: f64) -> Self {
        self.interval = Some(val);
        self
    }

    pub fn strike(mut self, val: f64) -> Self {
        self.strike = Some(val);
        self
    }

    pub fn range(mut self, val: String) -> Self {
        self.range = Some(val);
        self
    }

    pub fn from_date(mut self, val: chrono::NaiveDate) -> Self {
        self.from_date = Some(val);
        self
    }

    pub fn to_date(mut self, val: chrono::NaiveDate) -> Self {
        self.to_date = Some(val);
        self
    }

    pub fn volatility(mut self, val: f64) -> Self {
        self.volatility = Some(val);
        self
    }

    pub fn underlying_price(mut self, val: f64) -> Self {
        self.underlying_price = Some(val);
        self
    }

    pub fn interest_rate(mut self, val: f64) -> Self {
        self.interest_rate = Some(val);
        self
    }

    pub fn days_to_expiration(mut self, val: i64) -> Self {
        self.days_to_expiration = Some(val);
        self
    }

    pub fn exp_month(mut self, val: String) -> Self {
        self.exp_month = Some(val);
        self
    }

    pub fn option_type(mut self, val: String) -> Self {
        self.option_type = Some(val);
        self
    }

    pub fn entitlement(mut self, val: String) -> Self {
        self.entitlement = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req.query(&[
            ("symbol", self.symbol.clone()),
            ("strategy", self.strategy.clone()),
        ]);
        if let Some(x) = self.contract_type.clone() {
            req = req.query(&[("contractType", x)]);
        }
        if let Some(x) = self.strike_count {
            req = req.query(&[("strikeCount", x)]);
        }
        if let Some(x) = self.include_underlying_quote {
            req = req.query(&[("includeUnderlyingQuote", x)]);
        }
        if let Some(x) = self.interval {
            req = req.query(&[("interval", x)]);
        }
        if let Some(x) = self.strike {
            req = req.query(&[("strike", x)]);
        }
        if let Some(x) = self.range.clone() {
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
        if let Some(x) = self.exp_month.clone() {
            req = req.query(&[("expMonth", x)]);
        }
        if let Some(x) = self.option_type.clone() {
            req = req.query(&[("optionType", x)]);
        }
        if let Some(x) = self.entitlement.clone() {
            req = req.query(&[("entitlement", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<model::OptionChain, Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ErrorResponse>().await?;
            return Err(Error::ErrorResponse(error_response));
        }

        rsp.json::<model::OptionChain>()
            .await
            .map_err(std::convert::Into::into)
    }
}

#[derive(Debug)]
pub struct GetOptionExpirationChainRequest {
    req: RequestBuilder,

    symbol: String,
}

impl GetOptionExpirationChainRequest {
    pub(crate) fn new(req: RequestBuilder, symbol: String) -> Self {
        Self { req, symbol }
    }

    fn build(self) -> RequestBuilder {
        let req = self.req.query(&[("symbol", self.symbol.clone())]);

        req
    }

    pub async fn send(self) -> Result<model::ExpirationChain, Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ErrorResponse>().await?;
            return Err(Error::ErrorResponse(error_response));
        }

        rsp.json::<model::ExpirationChain>()
            .await
            .map_err(std::convert::Into::into)
    }
}

// pub(crate) struct PriceHistoryRequestBuilder {
// req: RequestBuilder,
// }

// pub(crate) struct MoverRequestBuilder {
// req: RequestBuilder,
// }

// pub(crate) struct MarketHourRequestBuilder {
// req: RequestBuilder,
// }

// pub(crate) struct InstrumentRequestBuilder {
// req: RequestBuilder,
// }

#[cfg(test)]
mod tests {
    use super::*;

    use mockito::Matcher;
    use pretty_assertions::assert_eq;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_quotes_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let symbols = vec!["symbol1".to_string(), "symbol2".to_string()];
        let fields = vec!["reference".to_string(), "regular".to_string()];
        let indicative = true;

        // Create a mock
        let mock = server
            .mock("GET", "/quotes")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbols".into(), symbols.join(",")),
                Matcher::UrlEncoded("fields".into(), fields.join(",")),
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
        let req = client.get(format!("{url}/quotes"));

        let mut req = GetQuotesRequest::new(req, symbols.clone());

        // check initial value
        assert_eq!(req.symbols, symbols);
        assert_eq!(req.fields, vec!["all"]);
        assert_eq!(req.indicative, None);

        // check setter
        req = req.fields(fields.clone());
        assert_eq!(req.fields, fields);
        req = req.indicative(indicative);
        assert_eq!(req.indicative, Some(indicative));

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.len(), 17);
    }

    #[tokio::test]
    async fn test_get_quote_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let symbol = "AAPL".to_string();
        let fields = vec!["reference".to_string(), "regular".to_string()];

        // Create a mock
        let mock = server
            .mock("GET", "/AAPL/quotes")
            .match_query(Matcher::UrlEncoded("fields".into(), fields.join(",")))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
					  "AAPL": {
						"assetMainType": "EQUITY",
						"symbol": "AAPL",
						"quoteType": "NBBO",
						"realtime": true,
						"ssid": 1973757747,
						"reference": {
						  "cusip": "037833100",
						  "description": "Apple Inc",
						  "exchange": "Q",
						  "exchangeName": "NASDAQ"
						},
						"quote": {
						  "52WeekHigh": 169,
						  "52WeekLow": 1.1,
						  "askMICId": "MEMX",
						  "askPrice": 168.41,
						  "askSize": 400,
						  "askTime": 1644854683672,
						  "bidMICId": "IEGX",
						  "bidPrice": 168.4,
						  "bidSize": 400,
						  "bidTime": 1644854683633,
						  "closePrice": 177.57,
						  "highPrice": 169,
						  "lastMICId": "XADF",
						  "lastPrice": 168.405,
						  "lastSize": 200,
						  "lowPrice": 167.09,
						  "mark": 168.405,
						  "markChange": -9.164999999999992,
						  "markPercentChange": -5.161344821760428,
						  "netChange": -9.165,
						  "netPercentChange": -5.161344821760428,
						  "openPrice": 167.37,
						  "quoteTime": 1644854683672,
						  "securityStatus": "Normal",
						  "totalVolume": 22361159,
						  "tradeTime": 1644854683408,
						  "volatility": 0.0347
						},
						"regular": {
						  "regularMarketLastPrice": 168.405,
						  "regularMarketLastSize": 2,
						  "regularMarketNetChange": -9.165,
						  "regularMarketPercentChange": -5.161344821760428,
						  "regularMarketTradeTime": 1644854683408
						},
						"fundamental": {
						  "avg10DaysVolume": 1,
						  "avg1YearVolume": 0,
						  "divAmount": 1.1,
						  "divFreq": 0,
						  "divPayAmount": 0,
						  "divYield": 1.1,
						  "eps": 0,
						  "fundLeverageFactor": 1.1,
						  "peRatio": 1.1
						}
					  }
					}"#,
            )
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!("{url}/{symbol}/quotes"));
        let mut req = GetQuoteRequest::new(req, symbol.clone());

        // check initial value
        assert_eq!(req.symbol, symbol);
        assert_eq!(req.fields, vec!["all"]);

        // check setter
        req = req.fields(fields.clone());
        assert_eq!(req.fields, fields);

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        match result {
            model::QuoteResponse::Equity(x) => assert_eq!(x.symbol, symbol),
            _ => panic!("not Equity"),
        }
    }

    #[tokio::test]
    async fn test_get_options_chains_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let symbol = "string".to_string();
        let contract_type = "CALL".to_string();
        let strike_count = 1;
        let include_underlying_quote = true;
        let strategy = "SINGLE".to_string();
        let interval = 1.1;
        let strike = 2.2;
        let range = "ITM".to_string();
        let from_date = chrono::NaiveDate::from_ymd_opt(2015, 3, 14).unwrap();
        let to_date = chrono::NaiveDate::from_ymd_opt(2015, 5, 14).unwrap();
        let volatility = 3.3;
        let underlying_price = 4.4;
        let interest_rate = 5.5;
        let days_to_expiration = 2;
        let exp_month = "JAN".to_string();
        let option_type = "option_type".to_string();
        let entitlement = "PN".to_string();

        // Create a mock
        let mock = server
            .mock("GET", "/chains")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbol".into(), symbol.clone()),
                Matcher::UrlEncoded("contractType".into(), contract_type.clone()),
                Matcher::UrlEncoded("strikeCount".into(), strike_count.to_string()),
                Matcher::UrlEncoded(
                    "includeUnderlyingQuote".into(),
                    include_underlying_quote.to_string(),
                ),
                Matcher::UrlEncoded("strategy".into(), strategy.clone()),
                Matcher::UrlEncoded("interval".into(), interval.to_string()),
                Matcher::UrlEncoded("strike".into(), strike.to_string()),
                Matcher::UrlEncoded("range".into(), range.clone()),
                Matcher::UrlEncoded("fromDate".into(), from_date.to_string()),
                Matcher::UrlEncoded("toDate".into(), to_date.to_string()),
                Matcher::UrlEncoded("volatility".into(), volatility.to_string()),
                Matcher::UrlEncoded("underlyingPrice".into(), underlying_price.to_string()),
                Matcher::UrlEncoded("interestRate".into(), interest_rate.to_string()),
                Matcher::UrlEncoded("daysToExpiration".into(), days_to_expiration.to_string()),
                Matcher::UrlEncoded("expMonth".into(), exp_month.clone()),
                Matcher::UrlEncoded("optionType".into(), option_type.clone()),
                Matcher::UrlEncoded("entitlement".into(), entitlement.clone()),
            ]))
            // .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/MarketData/OptionChain.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!("{url}/chains"));
        let mut req = GetOptionChainsRequest::new(req, symbol.clone());

        // check initial value
        assert_eq!(req.symbol, symbol);
        assert_eq!(req.contract_type, None);
        assert_eq!(req.strike_count, None);
        assert_eq!(req.include_underlying_quote, None);
        assert_eq!(req.strategy, "SINGLE".to_string());
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
        req = req.contract_type(contract_type.clone());
        assert_eq!(req.contract_type, Some(contract_type));
        req = req.strike_count(strike_count.clone());
        assert_eq!(req.strike_count, Some(strike_count));
        req = req.include_underlying_quote(include_underlying_quote.clone());
        assert_eq!(req.include_underlying_quote, Some(include_underlying_quote));
        req = req.strategy(strategy.clone());
        assert_eq!(req.strategy, strategy);
        req = req.interval(interval.clone());
        assert_eq!(req.interval, Some(interval));
        req = req.strike(strike.clone());
        assert_eq!(req.strike, Some(strike));
        req = req.range(range.clone());
        assert_eq!(req.range, Some(range));
        req = req.from_date(from_date.clone());
        assert_eq!(req.from_date, Some(from_date));
        req = req.to_date(to_date.clone());
        assert_eq!(req.to_date, Some(to_date));
        req = req.volatility(volatility.clone());
        assert_eq!(req.volatility, Some(volatility));
        req = req.underlying_price(underlying_price.clone());
        assert_eq!(req.underlying_price, Some(underlying_price));
        req = req.interest_rate(interest_rate.clone());
        assert_eq!(req.interest_rate, Some(interest_rate));
        req = req.days_to_expiration(days_to_expiration.clone());
        assert_eq!(req.days_to_expiration, Some(days_to_expiration));
        req = req.exp_month(exp_month.clone());
        assert_eq!(req.exp_month, Some(exp_month));
        req = req.option_type(option_type.clone());
        assert_eq!(req.option_type, Some(option_type));
        req = req.entitlement(entitlement.clone());
        assert_eq!(req.entitlement, Some(entitlement));

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.status, "string");
    }

    #[tokio::test]
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
                "/tests/model/MarketData/ExpirationChain.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!("{url}/expirationchain"));
        let req = GetOptionExpirationChainRequest::new(req, symbol.clone());

        // check initial value
        assert_eq!(req.symbol, symbol);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.expiration_list.len(), 18);
    }
}
