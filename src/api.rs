mod endpoints;
mod market_data;
mod trader;

use reqwest::Client;
use std::path::PathBuf;

use super::token::TokenChecker;
use crate::{error::Error, model};

#[derive(Debug)]
pub struct API {
    token_checker: TokenChecker,
    client: Client,
}

impl API {
    /// # Panics
    ///
    /// Will panic if no home dir
    pub async fn new(
        key: String,
        secret: String,
        callback_url: String,
        certs_dir: &str,
    ) -> Result<Self, Error> {
        let path = dirs::home_dir()
            .expect("home dir")
            .join(".credentials")
            .join("Schwab-rust.json");
        let certs_dir = PathBuf::from(certs_dir);

        let token_checker = TokenChecker::new(path, key, secret, callback_url, certs_dir).await?;
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
            &self.client,
            access_token,
            symbols,
        ))
    }

    pub async fn get_quote(&self, symbol: String) -> Result<market_data::GetQuoteRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(market_data::GetQuoteRequest::new(
            &self.client,
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
            &self.client,
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
            &self.client,
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
            &self.client,
            access_token,
            symbol,
        ))
    }

    pub async fn get_movers(&self, symbol: String) -> Result<market_data::GetMoversRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(market_data::GetMoversRequest::new(
            &self.client,
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
            &self.client,
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
            &self.client,
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
            &self.client,
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
            &self.client,
            access_token,
            cusip_id,
        ))
    }

    pub async fn get_account_numbers(&self) -> Result<trader::GetAccountNumbersRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(trader::GetAccountNumbersRequest::new(
            &self.client,
            access_token,
        ))
    }

    pub async fn get_accounts(&self) -> Result<trader::GetAccountsRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(trader::GetAccountsRequest::new(&self.client, access_token))
    }

    pub async fn get_account(
        &self,
        account_number: String,
    ) -> Result<trader::GetAccountRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(trader::GetAccountRequest::new(
            &self.client,
            access_token,
            account_number,
        ))
    }

    pub async fn get_account_orders(
        &self,
        account_number: String,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Result<trader::GetAccountOrdersRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(trader::GetAccountOrdersRequest::new(
            &self.client,
            access_token,
            account_number,
            from_entered_time,
            to_entered_time,
        ))
    }

    pub async fn post_account_order(
        &self,
        account_number: String,
        body: model::OrderRequest,
    ) -> Result<trader::PostAccountOrderRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(trader::PostAccountOrderRequest::new(
            &self.client,
            access_token,
            account_number,
            body,
        ))
    }

    pub async fn get_account_order(
        &self,
        account_number: String,
        order_id: i64,
    ) -> Result<trader::GetAccountOrderRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(trader::GetAccountOrderRequest::new(
            &self.client,
            access_token,
            account_number,
            order_id,
        ))
    }

    pub async fn delete_account_order(
        &self,
        account_number: String,
        order_id: i64,
    ) -> Result<trader::DeleteAccountOrderRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(trader::DeleteAccountOrderRequest::new(
            &self.client,
            access_token,
            account_number,
            order_id,
        ))
    }

    pub async fn put_account_order(
        &self,
        account_number: String,
        order_id: i64,
        body: model::OrderRequest,
    ) -> Result<trader::PutAccountOrderRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(trader::PutAccountOrderRequest::new(
            &self.client,
            access_token,
            account_number,
            order_id,
            body,
        ))
    }

    pub async fn get_accounts_orders(
        &self,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Result<trader::GetAccountsOrdersRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(trader::GetAccountsOrdersRequest::new(
            &self.client,
            access_token,
            from_entered_time,
            to_entered_time,
        ))
    }

    pub async fn post_accounts_preview_order(
        &self,
        account_number: String,
        body: model::PreviewOrder,
    ) -> Result<trader::PostAccountPreviewOrderRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(trader::PostAccountPreviewOrderRequest::new(
            &self.client,
            access_token,
            account_number,
            body,
        ))
    }

    pub async fn get_account_transactions(
        &self,
        account_number: String,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        types: String,
    ) -> Result<trader::GetAccountTransactions, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(trader::GetAccountTransactions::new(
            &self.client,
            access_token,
            account_number,
            start_date,
            end_date,
            types,
        ))
    }

    pub async fn get_account_transaction(
        &self,
        account_number: String,
        transaction_id: i64,
    ) -> Result<trader::GetAccountTransaction, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(trader::GetAccountTransaction::new(
            &self.client,
            access_token,
            account_number,
            transaction_id,
        ))
    }

    pub async fn get_user_preference(&self) -> Result<trader::GetUserPreferenceRequest, Error> {
        let access_token = self.token_checker.get_access_token().await?;

        Ok(trader::GetUserPreferenceRequest::new(
            &self.client,
            access_token,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn client() -> API {
        #[allow(clippy::option_env_unwrap)]
        let client_id = option_env!("SCHWAB_API_KEY").expect("There should be SCHWAB API KEY");
        #[allow(clippy::option_env_unwrap)]
        let secret = option_env!("SCHWAB_SECRET").expect("There should be SCHWAB SECRET");
        API::new(
            client_id.to_string(),
            secret.to_string(),
            "https://127.0.0.1:8080".to_string(),
            concat!(env!("CARGO_MANIFEST_DIR"), "tests/certs"),
        )
        .await
        .unwrap()
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
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

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
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

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
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

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
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

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
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

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
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

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
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

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
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

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
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

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
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

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_account_numbers() {
        let api = client().await;
        dbg!(api
            .get_account_numbers()
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_accounts() {
        let api = client().await;
        dbg!(api.get_accounts().await.unwrap().send().await.unwrap());
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_account() {
        let api = client().await;
        dbg!(api
            .get_account("account_number".into())
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_account_orders() {
        let api = client().await;
        dbg!(api
            .get_account_orders(
                "account_number".into(),
                chrono::NaiveDate::from_ymd_opt(2015, 1, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
                chrono::NaiveDate::from_ymd_opt(2016, 1, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap()
            )
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_post_account_order() {
        let api = client().await;
        dbg!(api
            .post_account_order("account_number".into(), model::OrderRequest::default())
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_account_order() {
        let api = client().await;
        dbg!(api
            .get_account_order("account_number".into(), 0)
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_delete_account_order() {
        let api = client().await;
        dbg!(api
            .delete_account_order("account_number".into(), 0)
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_put_account_order() {
        let api = client().await;
        dbg!(api
            .put_account_order("account_number".into(), 0, model::OrderRequest::default())
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_accounts_orders() {
        let api = client().await;
        dbg!(api
            .get_accounts_orders(
                chrono::NaiveDate::from_ymd_opt(2015, 1, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
                chrono::NaiveDate::from_ymd_opt(2016, 1, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap()
            )
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_post_accounts_preview_order() {
        let api = client().await;
        dbg!(api
            .post_accounts_preview_order("account_number".into(), model::PreviewOrder::default())
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_account_transactions() {
        let api = client().await;
        dbg!(api
            .get_account_transactions(
                "account_number".into(),
                chrono::NaiveDate::from_ymd_opt(2015, 1, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
                chrono::NaiveDate::from_ymd_opt(2016, 1, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
                "TRADE".to_string()
            )
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_account_transaction() {
        let api = client().await;
        dbg!(api
            .get_account_transaction("account_number".into(), 0)
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_user_preference() {
        let api = client().await;
        dbg!(api
            .get_user_preference()
            .await
            .unwrap()
            .send()
            .await
            .unwrap());
    }
}
