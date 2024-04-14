use reqwest::{Client, StatusCode};
use std::collections::HashMap;

use super::{
    endpoints::{self, Endpoint, EndpointPriceHistory, EndpointQuote},
    model,
    token::TokenChecker,
};
use crate::error::Error;

#[derive(Debug)]
pub(crate) struct API {
    token_checker: TokenChecker,
    client: Client,
}

impl API {
    pub(crate) async fn new(client_id: String, secret: String) -> Result<Self, Error> {
        let path = dirs::home_dir()
            .expect("home dir")
            .join(".credentials")
            .join("Schwab-rust.json");

        let token_checker = TokenChecker::new(path, client_id, secret).await?;
        let client = Client::new();

        Ok(API {
            token_checker,
            client,
        })
    }

    pub(crate) async fn quotes(
        &self,
        symbols: &[&str],
    ) -> Result<HashMap<String, model::quote::Quote>, Error> {
        if symbols.is_empty() {
            return Ok(HashMap::new());
        }

        let access_token = self.token_checker.get_access_token().await?;
        let req = self
            .client
            .get(Endpoint::Quote(EndpointQuote::Quotes).url_endpoint())
            .bearer_auth(access_token)
            .query(&[("symbols", symbols.join(","))]);

        let rsp = req.send().await?;
        let status = rsp.status();
        if status != StatusCode::OK {
            return Err(Error::HttpRequestFailed(format!("{symbols:?}"), status));
        }
        let response_text = rsp.text().await?;
        if response_text == "{}" {
            return Err(Error::EmptyResponseText(format!("{symbols:?}")));
        }

        if let Ok(json) =
            serde_json::from_str::<HashMap<String, model::quote::QEquity>>(&response_text)
        {
            return Ok(json
                .into_iter()
                .map(|(symbol, x)| (symbol, x.into()))
                .collect());
        }
        if let Ok(json) =
            serde_json::from_str::<HashMap<String, model::quote::QFund>>(&response_text)
        {
            return Ok(json
                .into_iter()
                .map(|(symbol, x)| (symbol, x.into()))
                .collect());
        }
        if let Ok(json) =
            serde_json::from_str::<HashMap<String, model::quote::QIndex>>(&response_text)
        {
            return Ok(json
                .into_iter()
                .map(|(symbol, x)| (symbol, x.into()))
                .collect());
        }
        if let Ok(json) =
            serde_json::from_str::<HashMap<String, model::quote::QOption>>(&response_text)
        {
            return Ok(json
                .into_iter()
                .map(|(symbol, x)| (symbol, x.into()))
                .collect());
        }
        if let Ok(json) =
            serde_json::from_str::<HashMap<String, model::quote::QGeneral>>(&response_text)
        {
            return Ok(json
                .into_iter()
                .map(|(symbol, x)| (symbol, x.into()))
                .collect());
        }

        Err(Error::NoMatchJsonFormat(response_text))
    }

    pub(crate) async fn history(
        &self,
        symbol: &str,
    ) -> Result<model::pricehistory::History, Error> {
        let access_token = self.token_checker.get_access_token().await?;
        let req = self
            .client
            .get(Endpoint::PriceHistory(EndpointPriceHistory::PriceHistory).url_endpoint())
            .bearer_auth(access_token)
            .query(&[
                ("periodType", "year"),
                ("period", "20"),
                ("frequencyType", "daily"),
                ("frequency", "1"),
            ]);

        let rsp = req.send().await?;
        // let response_text = rsp.text().await?;
        // println!("{}", &response_text);
        // panic!();
        rsp.json().await.map_err(std::convert::Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn tda_client() -> API {
        #[allow(clippy::option_env_unwrap)]
        let client_id = option_env!("SCHWAB_API_KEY").expect("There should be SCHWAB API KEY");
        let secret = option_env!("SCHWAB_SECRET").expect("There should be SCHWAB SECRET");
        API::new(client_id.to_string(), secret.to_string())
            .await
            .unwrap()
    }

    #[tokio::test]
    #[cfg_attr(
        not(feature = "test_tda"),
        ignore = r#"pass feature "test_tda". There should be SCHWAB_API_KEY in env"#
    )]
    async fn test_quotes() {
        let tdaclient = tda_client().await;
        let prices = tdaclient.quotes(&["VTI", "VBR"]).await.unwrap();
        // println!("{:?}", &prices);

        let key = "VTI";
        let price = prices.get(key).unwrap();
        println!("{:?}", &price);
        assert_eq!(key, price.symbol());
        assert!(price.market_price().is_some());
        assert!(price.open_price().is_some());
        assert!(price.high_price().is_some());
        assert!(price.low_price().is_some());
        assert!(price.close_price().is_some());
        assert!(price.volume().is_some());

        let key = "VBR";
        let price = prices.get(key).unwrap();
        assert_eq!(key, price.symbol());
        assert!(price.close_price().is_some());
    }

    #[tokio::test]
    #[cfg_attr(
        not(feature = "test_tda"),
        ignore = r#"pass feature "test_tda". There should be SCHWAB_API_KEY in env"#
    )]
    async fn test_history() {
        let tdaclient = tda_client().await;
        let history = tdaclient.history("VTI").await.unwrap();
        // dbg!(&history);
        assert!(!history.candles.is_empty());
        println!("{:?}", history.candles[0]);
    }

    #[tokio::test]
    #[cfg_attr(
        not(feature = "test_tda"),
        ignore = r#"pass feature "test_tda". There should be SCHWAB_API_KEY in env"#
    )]
    async fn test_history_empty() {
        let tdaclient = tda_client().await;
        let history = tdaclient.history("0050.TW").await.unwrap();
        assert!(history.empty);
    }
}
