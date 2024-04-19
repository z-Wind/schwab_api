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

        Ok(market_data::GetQuotesRequest::new(
            self.client.clone(),
            access_token,
            symbols,
        ))
    }

    pub async fn get_quote(&self, symbol: String) -> Result<market_data::GetQuoteRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(market_data::GetQuoteRequest::new(
            self.client.clone(),
            access_token,
            symbol,
        ))
    }

    pub async fn get_option_chains(
        &self,
        symbol: String,
    ) -> Result<market_data::GetOptionChainsRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(market_data::GetOptionChainsRequest::new(
            self.client.clone(),
            access_token,
            symbol,
        ))
    }

    pub async fn get_option_expiration_chain(
        &self,
        symbol: String,
    ) -> Result<market_data::GetOptionExpirationChainRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(market_data::GetOptionExpirationChainRequest::new(
            self.client.clone(),
            access_token,
            symbol,
        ))
    }

    pub async fn get_price_history(
        &self,
        symbol: String,
    ) -> Result<market_data::GetPriceHistoryRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(market_data::GetPriceHistoryRequest::new(
            self.client.clone(),
            access_token,
            symbol,
        ))
    }

    pub async fn get_movers(&self, symbol: String) -> Result<market_data::GetMoversRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(market_data::GetMoversRequest::new(
            self.client.clone(),
            access_token,
            symbol,
        ))
    }

    pub async fn get_markets(
        &self,
        markets: Vec<String>,
    ) -> Result<market_data::GetMarketsRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(market_data::GetMarketsRequest::new(
            self.client.clone(),
            access_token,
            markets,
        ))
    }

    pub async fn get_market(
        &self,
        market_id: String,
    ) -> Result<market_data::GetMarketRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(market_data::GetMarketRequest::new(
            self.client.clone(),
            access_token,
            market_id,
        ))
    }

    pub async fn get_instrucments(
        &self,
        symbol: String,
        projection: String,
    ) -> Result<market_data::GetInstrucmentsRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(market_data::GetInstrucmentsRequest::new(
            self.client.clone(),
            access_token,
            symbol,
            projection,
        ))
    }

    pub async fn get_instrucment(
        &self,
        cusip_id: String,
    ) -> Result<market_data::GetInstrucmentRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(market_data::GetInstrucmentRequest::new(
            self.client.clone(),
            access_token,
            cusip_id,
        ))
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

    #[tokio::test]
    async fn test_get_option_chains() {
        let api = client().await;
        dbg!(api
            .get_option_chains("AAPL".into())
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_get_option_expiration_chain() {
        let api = client().await;
        dbg!(api
            .get_option_chains("AAPL".into())
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_get_price_history() {
        let api = client().await;
        dbg!(api
            .get_price_history("AAPL".into())
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_get_movers() {
        let api = client().await;
        dbg!(api
            .get_movers("$DJI".into())
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_get_markets() {
        let api = client().await;
        dbg!(api
            .get_markets(vec!["equity".to_string(), "option".to_string()])
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_get_market() {
        let api = client().await;
        dbg!(api
            .get_market("equity".into())
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_get_instrucments() {
        let api = client().await;
        dbg!(api
            .get_instrucments("VTI".into(), "symbol-search".into())
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_get_instrucment() {
        let api = client().await;
        dbg!(api
            .get_instrucment("037833100".into())
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }
}
