mod endpoints;
mod market_data;
mod trader;

use reqwest::Client;

use super::token::TokenChecker;
use crate::error::Error;

#[derive(Debug)]
pub struct API {
    token_checker: TokenChecker,
    client: Client,
}

impl API {
    /// # Panics
    ///
    /// Will panic if no home dir
    pub async fn new(client_id: String, secret: String) -> Result<Self, Error> {
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

    pub async fn get_quotes(
        &self,
        symbols: Vec<String>,
    ) -> Result<market_data::GetQuotesRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;
        let req = self
            .client
            .get(endpoints::Endpoint::Quote(endpoints::EndpointQuote::Quotes).url_endpoint())
            .bearer_auth(access_token);
        Ok(market_data::GetQuotesRequest::new(req, symbols))
    }

    pub async fn get_quote(&self, symbol: String) -> Result<market_data::GetQuoteRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;
        let req = self
            .client
            .get(
                endpoints::Endpoint::Quote(endpoints::EndpointQuote::Quote { symbol_id: &symbol })
                    .url_endpoint(),
            )
            .bearer_auth(access_token);
        Ok(market_data::GetQuoteRequest::new(req, symbol))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn client() -> API {
        #[allow(clippy::option_env_unwrap)]
        let client_id = option_env!("SCHWAB_API_KEY").expect("There should be SCHWAB API KEY");
        let secret = option_env!("SCHWAB_SECRET").expect("There should be SCHWAB SECRET");
        API::new(client_id.to_string(), secret.to_string())
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_get_quotes() {
        let api = client().await;
        dbg!(api
            .get_quotes(vec!["VTI".into(), "VBR".into()])
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_get_quote() {
        let api = client().await;
        dbg!(api
            .get_quote("VTI".into())
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }
}
