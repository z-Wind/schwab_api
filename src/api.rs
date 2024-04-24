//! Structs and utilities for handling API methods.

mod endpoints;
pub mod market_data;
pub mod parameter;
pub mod trader;

use reqwest::Client;

use crate::token::Tokener;
use crate::{error::Error, model};
use parameter::{Market, Projection, TransactionType};

/// Interacting with the Schwab API.
#[derive(Debug)]
pub struct API<T: Tokener> {
    tokener: T,
    client: Client,
}

impl<T: Tokener> API<T> {
    /// Create API Struct
    pub fn new(tokener: T) -> Result<Self, Error> {
        let client = Client::new();

        Ok(API { tokener, client })
    }

    pub async fn get_quotes(
        &self,
        symbols: Vec<String>,
    ) -> Result<market_data::GetQuotesRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetQuotesRequest::new(
            &self.client,
            access_token,
            symbols,
        ))
    }

    pub async fn get_quote(&self, symbol: String) -> Result<market_data::GetQuoteRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

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
        let access_token = self.tokener.get_access_token().await?;

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
        let access_token = self.tokener.get_access_token().await?;

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
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetPriceHistoryRequest::new(
            &self.client,
            access_token,
            symbol,
        ))
    }

    /// `symbol`
    ///
    /// Index Symbol
    ///
    /// Available values : `$DJI`, `$COMPX`, `$SPX`, `NYSE`, `NASDAQ`, `OTCBB`, `INDEX_ALL`, `EQUITY_ALL`, `OPTION_ALL`, `OPTION_PUT`, `OPTION_CALL`
    ///
    /// Example : `$DJI`
    pub async fn get_movers(&self, symbol: String) -> Result<market_data::GetMoversRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetMoversRequest::new(
            &self.client,
            access_token,
            symbol,
        ))
    }

    /// `markets`
    ///
    /// List of markets
    ///
    /// Available values : `equity`, `option`, `bond`, `future`, `forex`
    pub async fn get_markets(
        &self,
        markets: Vec<Market>,
    ) -> Result<market_data::GetMarketsRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetMarketsRequest::new(
            &self.client,
            access_token,
            markets,
        ))
    }

    /// `market_id`
    ///
    /// Available values : `equity`, `option`, `bond`, `future`, `forex`
    pub async fn get_market(
        &self,
        market_id: Market,
    ) -> Result<market_data::GetMarketRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetMarketRequest::new(
            &self.client,
            access_token,
            market_id,
        ))
    }

    /// `projection`
    ///
    /// search by
    ///
    /// Available values : `symbol-search`, `symbol-regex`, `desc-search`, `desc-regex`, `search`, `fundamental`
    pub async fn get_instrucments(
        &self,
        symbol: String,
        projection: Projection,
    ) -> Result<market_data::GetInstrucmentsRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetInstrucmentsRequest::new(
            &self.client,
            access_token,
            symbol,
            projection,
        ))
    }

    /// `cusip_id`
    ///
    /// cusip of a security
    pub async fn get_instrucment(
        &self,
        cusip_id: String,
    ) -> Result<market_data::GetInstrucmentRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetInstrucmentRequest::new(
            &self.client,
            access_token,
            cusip_id,
        ))
    }

    pub async fn get_account_numbers(&self) -> Result<trader::GetAccountNumbersRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::GetAccountNumbersRequest::new(
            &self.client,
            access_token,
        ))
    }

    pub async fn get_accounts(&self) -> Result<trader::GetAccountsRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::GetAccountsRequest::new(&self.client, access_token))
    }

    pub async fn get_account(
        &self,
        account_number: String,
    ) -> Result<trader::GetAccountRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::GetAccountRequest::new(
            &self.client,
            access_token,
            account_number,
        ))
    }

    /// `from_entered_time`
    ///
    /// Specifies that no orders entered before this time should be returned.
    ///
    /// Date must be within 60 days from today's date.
    ///
    /// `to_entered_time`
    ///
    /// Specifies that no orders entered after this time should be returned.
    pub async fn get_account_orders(
        &self,
        account_number: String,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Result<trader::GetAccountOrdersRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::GetAccountOrdersRequest::new(
            &self.client,
            access_token,
            account_number,
            from_entered_time,
            to_entered_time,
        ))
    }

    /// `account_number`
    ///
    /// The encrypted ID of the account
    pub async fn post_account_order(
        &self,
        account_number: String,
        body: model::OrderRequest,
    ) -> Result<trader::PostAccountOrderRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::PostAccountOrderRequest::new(
            &self.client,
            access_token,
            account_number,
            body,
        ))
    }

    /// `account_number`
    ///
    /// The encrypted ID of the account
    ///
    /// `order_id`
    ///
    /// The ID of the order being retrieved.
    pub async fn get_account_order(
        &self,
        account_number: String,
        order_id: i64,
    ) -> Result<trader::GetAccountOrderRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::GetAccountOrderRequest::new(
            &self.client,
            access_token,
            account_number,
            order_id,
        ))
    }

    /// `account_number`
    ///
    /// The encrypted ID of the account
    ///
    /// `order_id`
    ///
    /// The ID of the order being retrieved.
    pub async fn delete_account_order(
        &self,
        account_number: String,
        order_id: i64,
    ) -> Result<trader::DeleteAccountOrderRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::DeleteAccountOrderRequest::new(
            &self.client,
            access_token,
            account_number,
            order_id,
        ))
    }

    /// `account_number`
    ///
    /// The encrypted ID of the account
    ///
    /// `order_id`
    ///
    /// The ID of the order being retrieved.
    pub async fn put_account_order(
        &self,
        account_number: String,
        order_id: i64,
        body: model::OrderRequest,
    ) -> Result<trader::PutAccountOrderRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::PutAccountOrderRequest::new(
            &self.client,
            access_token,
            account_number,
            order_id,
            body,
        ))
    }

    /// `from_entered_time`
    ///
    /// Specifies that no orders entered before this time should be returned.
    ///
    /// Date must be within 60 days from today's date.
    ///
    /// `to_entered_time`
    ///
    /// Specifies that no orders entered after this time should be returned.
    pub async fn get_accounts_orders(
        &self,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Result<trader::GetAccountsOrdersRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::GetAccountsOrdersRequest::new(
            &self.client,
            access_token,
            from_entered_time,
            to_entered_time,
        ))
    }

    /// `account_number`
    ///
    /// The encrypted ID of the account
    pub async fn post_accounts_preview_order(
        &self,
        account_number: String,
        body: model::PreviewOrder,
    ) -> Result<trader::PostAccountPreviewOrderRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::PostAccountPreviewOrderRequest::new(
            &self.client,
            access_token,
            account_number,
            body,
        ))
    }

    /// `account_number`
    ///
    /// The encrypted ID of the account
    ///
    /// `start_date`
    ///
    /// Specifies that no transactions entered before this time should be returned.
    ///
    /// Date must be within 60 days from today's date.
    ///
    /// `end_date`
    ///
    /// Specifies that no transactions entered after this time should be returned.
    ///
    /// `types`
    ///
    /// Specifies that only transactions of this status should be returned.
    ///
    /// Available values : `TRADE`, `RECEIVE_AND_DELIVER`, `DIVIDEND_OR_INTEREST`, `ACH_RECEIPT`, `ACH_DISBURSEMENT`, `CASH_RECEIPT`, `CASH_DISBURSEMENT`, `ELECTRONIC_FUND`, `WIRE_OUT`, `WIRE_IN`, `JOURNAL`, `MEMORANDUM`, `MARGIN_CALL`, `MONEY_MARKET`, `SMA_ADJUSTMENT`
    pub async fn get_account_transactions(
        &self,
        account_number: String,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        types: TransactionType,
    ) -> Result<trader::GetAccountTransactions, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::GetAccountTransactions::new(
            &self.client,
            access_token,
            account_number,
            start_date,
            end_date,
            types,
        ))
    }

    /// `account_number`
    ///
    /// The encrypted ID of the account
    ///
    /// `transaction_id`
    ///
    /// The ID of the transaction being retrieved.
    pub async fn get_account_transaction(
        &self,
        account_number: String,
        transaction_id: i64,
    ) -> Result<trader::GetAccountTransaction, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::GetAccountTransaction::new(
            &self.client,
            access_token,
            account_number,
            transaction_id,
        ))
    }

    pub async fn get_user_preference(&self) -> Result<trader::GetUserPreferenceRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::GetUserPreferenceRequest::new(
            &self.client,
            access_token,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use crate::token::TokenChecker;

    async fn client() -> API<TokenChecker> {
        #[allow(clippy::option_env_unwrap)]
        let key = option_env!("SCHWAB_API_KEY")
            .expect("There should be SCHWAB API KEY")
            .to_string();
        #[allow(clippy::option_env_unwrap)]
        let secret = option_env!("SCHWAB_SECRET")
            .expect("There should be SCHWAB SECRET")
            .to_string();

        let path = dirs::home_dir()
            .expect("home dir")
            .join(".credentials")
            .join("Schwab-rust.json");

        let certs_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "tests/certs"));

        let callback_url = "https://127.0.0.1:8080".to_string();

        let token_checker = TokenChecker::new(path, key, secret, callback_url, certs_dir)
            .await
            .unwrap();

        API::new(token_checker).unwrap()
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
            .get_markets(vec![Market::Equity, Market::Option])
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
            .get_market(Market::Equity)
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
            .get_instrucments("VTI".into(), Projection::SymbolSearch)
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
                TransactionType::Trade,
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
