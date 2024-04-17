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
    indicative: bool,
}

impl GetQuotesRequest {
    pub(crate) fn new(req: RequestBuilder, symbols: Vec<String>) -> Self {
        Self {
            req,
            symbols,
            fields: vec!["all".to_string()],
            indicative: false,
        }
    }

    pub fn fields(mut self, val: Vec<String>) -> Self {
        self.fields = val;
        self
    }

    pub fn indicative(mut self, val: bool) -> Self {
        self.indicative = val;
        self
    }

    fn build(mut self) -> Self {
        self.req = self.req.query(&[
            ("symbols", self.symbols.join(",")),
            ("fields", self.fields.join(",")),
            ("indicative", self.indicative.to_string()),
        ]);
        self
    }

    pub async fn send(self) -> Result<HashMap<String, model::QuoteResponse>, Error> {
        let rsp = self.build().req.send().await?;

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

    fn build(mut self) -> Self {
        self.req = self.req.query(&[("fields", self.fields.join(","))]);
        self
    }

    pub async fn send(self) -> Result<model::QuoteResponse, Error> {
        let quote = self.build();
        let rsp = quote.req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ErrorResponse>().await?;
            return Err(Error::ErrorResponse(error_response));
        }

        let mut map = rsp.json::<HashMap<String, model::QuoteResponse>>().await?;
        Ok(map.remove(&quote.symbol).expect("must exist"))
    }
}

// pub(crate) struct OptionChainRequestBuilder {
// req: RequestBuilder,
// }

// pub(crate) struct OptionExpirationChainRequestBuilder {
// req: RequestBuilder,
// }

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

        // Create a mock
        let mock = server
            .mock("GET", "/quotes")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbols".into(), "symbol1,symbol2".into()),
                Matcher::UrlEncoded("fields".into(), "reference,regular".into()),
                Matcher::UrlEncoded("indicative".into(), "true".into()),
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
        let symbols = vec!["symbol1".to_string(), "symbol2".to_string()];
        let mut req = GetQuotesRequest::new(req, symbols.clone());

        assert_eq!(req.symbols, symbols);
        assert_eq!(req.fields, vec!["all"]);
        assert_eq!(req.indicative, false);

        let fields = vec!["reference".to_string(), "regular".to_string()];
        req = req.fields(fields.clone());
        assert_eq!(req.fields, fields);

        req = req.indicative(true);
        assert_eq!(req.indicative, true);

        dbg!(&req);
        let result = req.send().await.unwrap();
        mock.assert_async().await;
        assert_eq!(result.len(), 17);
    }

    #[tokio::test]
    async fn test_get_quote_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // Create a mock
        let symbol = "AAPL".to_string();
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

        assert_eq!(req.symbol, symbol);
        assert_eq!(req.fields, vec!["all"]);

        let fields = vec!["reference".to_string(), "regular".to_string()];
        req = req.fields(fields.clone());
        assert_eq!(req.fields, fields);

        dbg!(&req);
        let result = req.send().await.unwrap();
        mock.assert_async().await;
        match result {
            model::QuoteResponse::Equity(x) => assert_eq!(x.symbol, symbol),
            _ => panic!("not Equity"),
        }
    }
}
