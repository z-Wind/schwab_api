//! Structs and utilities for handling API methods.

mod endpoints;
pub mod market_data;
pub mod parameter;
pub mod trader;

use reqwest::Client;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::instrument;

use crate::token::Tokener;
use crate::{error::Error, model};
use parameter::{Market, Projection, TransactionType};

/// Interacting with the Schwab API.
#[derive(Debug)]
pub struct Api<T: Tokener> {
    pub tokener: T,
    client: Client,
}

impl<T: Tokener> Api<T> {
    /// Create API Struct
    #[instrument(skip(tokener, client))]
    pub async fn new(tokener: T, client: Client) -> Result<Self, Error> {
        tracing::info!("initializing Schwab API client");

        let api = Api { tokener, client };

        tracing::debug!("verifying API access with test quote request");
        if (api.get_quote("AAPL".to_string()).await?.send().await).is_err() {
            tracing::warn!("initial API access failed; forcing re-authorization");
            api.tokener.redo_authorization().await?;
        }

        tracing::info!("Schwab API client initialized successfully");
        Ok(api)
    }

    #[instrument(skip(self), fields(symbol_count = symbols.len()))]
    pub async fn get_quotes(
        &self,
        symbols: Vec<String>,
    ) -> Result<market_data::GetQuotesRequest, Error> {
        tracing::debug!("building multi-quote request");
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetQuotesRequest::new(
            &self.client,
            access_token,
            symbols,
        ))
    }

    #[instrument(skip(self), fields(symbol = %symbol))]
    pub async fn get_quote(&self, symbol: String) -> Result<market_data::GetQuoteRequest, Error> {
        tracing::debug!("building quote request");
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetQuoteRequest::new(
            &self.client,
            access_token,
            symbol,
        ))
    }

    #[instrument(skip(self), fields(symbol = %symbol))]
    pub async fn get_option_chains(
        &self,
        symbol: String,
    ) -> Result<market_data::GetOptionChainsRequest, Error> {
        tracing::debug!("building option chains request");
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetOptionChainsRequest::new(
            &self.client,
            access_token,
            symbol,
        ))
    }

    #[instrument(skip(self), fields(symbol = %symbol))]
    pub async fn get_option_expiration_chain(
        &self,
        symbol: String,
    ) -> Result<market_data::GetOptionExpirationChainRequest, Error> {
        tracing::debug!("building option expiration chain request");
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetOptionExpirationChainRequest::new(
            &self.client,
            access_token,
            symbol,
        ))
    }

    #[instrument(skip(self), fields(symbol = %symbol))]
    pub async fn get_price_history(
        &self,
        symbol: String,
    ) -> Result<market_data::GetPriceHistoryRequest, Error> {
        tracing::debug!("building price history request");
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
    #[instrument(skip(self), fields(symbol = %symbol))]
    pub async fn get_movers(&self, symbol: String) -> Result<market_data::GetMoversRequest, Error> {
        tracing::debug!("building movers request");
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
    #[instrument(skip(self), fields(market_count = markets.len()))]
    pub async fn get_markets(
        &self,
        markets: Vec<Market>,
    ) -> Result<market_data::GetMarketsRequest, Error> {
        tracing::debug!("building markets request");
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
    #[instrument(skip(self), fields(market_id = ?market_id))]
    pub async fn get_market(
        &self,
        market_id: Market,
    ) -> Result<market_data::GetMarketRequest, Error> {
        tracing::debug!("building market request");
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
    #[instrument(skip(self), fields(symbol = %symbol, projection = ?projection))]
    pub async fn get_instruments(
        &self,
        symbol: String,
        projection: Projection,
    ) -> Result<market_data::GetInstrumentsRequest, Error> {
        tracing::debug!("building instruments search request");
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetInstrumentsRequest::new(
            &self.client,
            access_token,
            symbol,
            projection,
        ))
    }

    /// `cusip_id`
    ///
    /// cusip of a security
    #[instrument(skip(self), fields(cusip_id = %cusip_id))]
    pub async fn get_instrument(
        &self,
        cusip_id: String,
    ) -> Result<market_data::GetInstrumentRequest, Error> {
        tracing::debug!("building instrument request");
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetInstrumentRequest::new(
            &self.client,
            access_token,
            cusip_id,
        ))
    }

    #[instrument(skip(self))]
    pub async fn get_account_numbers(&self) -> Result<trader::GetAccountNumbersRequest, Error> {
        tracing::debug!("building account numbers request");
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::GetAccountNumbersRequest::new(
            &self.client,
            access_token,
        ))
    }

    #[instrument(skip(self))]
    pub async fn get_accounts(&self) -> Result<trader::GetAccountsRequest, Error> {
        tracing::debug!("building accounts request");
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::GetAccountsRequest::new(&self.client, access_token))
    }

    #[instrument(skip(self, account_number))]
    pub async fn get_account(
        &self,
        account_number: String,
    ) -> Result<trader::GetAccountRequest, Error> {
        tracing::debug!("building account request");
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
    #[instrument(skip(self, account_number))]
    pub async fn get_account_orders(
        &self,
        account_number: String,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Result<trader::GetAccountOrdersRequest, Error> {
        tracing::debug!("building account orders request");
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
    #[instrument(skip(self, account_number, body))]
    pub async fn post_account_order(
        &self,
        account_number: String,
        body: model::OrderRequest,
    ) -> Result<trader::PostAccountOrderRequest, Error> {
        tracing::debug!("building post order request");
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
    #[instrument(skip(self, account_number))]
    pub async fn get_account_order(
        &self,
        account_number: String,
        order_id: i64,
    ) -> Result<trader::GetAccountOrderRequest, Error> {
        tracing::debug!("building get account order request");
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
    #[instrument(skip(self, account_number))]
    pub async fn delete_account_order(
        &self,
        account_number: String,
        order_id: i64,
    ) -> Result<trader::DeleteAccountOrderRequest, Error> {
        tracing::debug!("building delete order request");
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
    #[instrument(skip(self, account_number, body))]
    pub async fn put_account_order(
        &self,
        account_number: String,
        order_id: i64,
        body: model::OrderRequest,
    ) -> Result<trader::PutAccountOrderRequest, Error> {
        tracing::debug!("building update order request");
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
    #[instrument(skip(self))]
    pub async fn get_accounts_orders(
        &self,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Result<trader::GetAccountsOrdersRequest, Error> {
        tracing::debug!("building accounts orders request");
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
    #[instrument(skip(self, account_number, body))]
    pub async fn post_accounts_preview_order(
        &self,
        account_number: String,
        body: model::OrderRequest,
    ) -> Result<trader::PostAccountPreviewOrderRequest, Error> {
        tracing::debug!("building preview order request");
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
    #[instrument(skip(self, account_number), fields(transaction_type = ?types))]
    pub async fn get_account_transactions(
        &self,
        account_number: String,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        types: TransactionType,
    ) -> Result<trader::GetAccountTransactions, Error> {
        tracing::debug!("building account transactions request");
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
    #[instrument(skip(self, account_number))]
    pub async fn get_account_transaction(
        &self,
        account_number: String,
        transaction_id: i64,
    ) -> Result<trader::GetAccountTransaction, Error> {
        tracing::debug!("building account transaction request");
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::GetAccountTransaction::new(
            &self.client,
            access_token,
            account_number,
            transaction_id,
        ))
    }

    #[instrument(skip(self))]
    pub async fn get_user_preference(&self) -> Result<trader::GetUserPreferenceRequest, Error> {
        tracing::debug!("building user preference request");
        let access_token = self.tokener.get_access_token().await?;

        Ok(trader::GetUserPreferenceRequest::new(
            &self.client,
            access_token,
        ))
    }
}

#[instrument(skip(json), fields(model = %model))]
fn save_raw_json(folder: &str, model: &str, json: &str) {
    if json.trim().is_empty() {
        tracing::warn!("JSON content is empty; skipping file save.");
        return;
    }

    if let Err(e) = fs::create_dir_all(folder) {
        tracing::error!(
            directory = %folder,
            error = %e,
            "Failed to create directory"
        );
        return;
    }

    let formatted_json = serde_json::from_str::<serde_json::Value>(json)
        .and_then(|parsed| serde_json::to_string_pretty(&parsed))
        .unwrap_or_else(|_| json.to_string());

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    let file_name = format!("error_{}_{}.json", model, timestamp);
    let file_path = std::path::Path::new(folder).join(file_name);

    if let Err(e) = fs::write(&file_path, formatted_json) {
        tracing::error!(
            path = %file_path.display(),
            error = %e,
            "Failed to save error JSON file"
        );
    } else {
        tracing::debug!(
            path = %file_path.display(),
            "Error JSON saved successfully"
        );
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use pretty_assertions::assert_eq;
    use std::path::Path;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use test_log::test;

    use crate::model::trader::order::ExecutionType;
    use crate::model::trader::order_request::InstrumentRequest;
    use crate::model::trader::preview_order::Instruction;
    use crate::token::TokenChecker;
    use crate::token::channel_messenger::ChannelMessenger;
    use crate::token::channel_messenger::compound_messenger::CompoundMessenger;
    use crate::token::channel_messenger::local_server::LocalServerMessenger;
    use crate::token::channel_messenger::stdio_messenger::StdioMessenger;

    use super::*;

    async fn client() -> Api<TokenChecker<impl ChannelMessenger>> {
        #[allow(clippy::option_env_unwrap)]
        let key = option_env!("SCHWAB_API_KEY")
            .expect("The environment variable SCHWAB_API_KEY sholud be set")
            .to_string();

        #[allow(clippy::option_env_unwrap)]
        let secret = option_env!("SCHWAB_SECRET")
            .expect("The environment variable SCHWAB_SECRET sholud be set")
            .to_string();

        #[allow(clippy::option_env_unwrap)]
        let callback_url = option_env!("SCHWAB_CALLBACK_URL")
            .expect("The environment variable SCHWAB_CALLBACK_URL sholud be set")
            .to_string();

        let path = dirs::home_dir()
            .expect("home dir")
            .join(".credentials")
            .join("Schwab-rust.json");

        let certs_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/certs"));
        let messenger = CompoundMessenger::new(
            LocalServerMessenger::new(&certs_dir).await.unwrap(),
            StdioMessenger::new(),
        );

        let client = Client::new();
        let token_checker = TokenChecker::new_with_custom_auth(
            path,
            key,
            secret,
            callback_url,
            client.clone(),
            messenger,
        )
        .await
        .unwrap();

        Api::new(token_checker, client).await.unwrap()
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_quotes() {
        let api = client().await;
        let req = api
            .get_quotes(vec![
                // Bond #unsupported
                //"^IRX".into(),
                // EQUITY
                "AAPL".into(),
                // FOREX
                // https://www.schwab.com/forex/what-is-forex#bcn-table-206441
                "EUR/USD".into(),
                // FUTURE
                // https://help.streetsmart.schwab.com/edge/1.68/Content/Futures%20Symbols.htm
                // '/' + 'root symbol' + 'month code' + 'year code'
                "/ESZ24".into(),
                // FUTURE_OPTION #unsupported
                //"ESZ24.CME".into(),
                // INDEX
                "$SPX".into(),
                // MUTUAL_FUND
                "AAAIX".into(),
                // OPTION
                // Symbol (max. 6 characters) + Yr (YY) + Mo (MM) + Day (DD) + Call or Put (C/P) + Strike Price (#####.###) listed with five digits before the decimal and three digits following the decimal
                // "AAPL  240517C00100000".into(),
                get_option_chain("AAPL".to_string()).await,
            ])
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_quote() {
        let symbols = vec![
            // Bond
            // "^IRX".to_string(),
            // EQUITY
            "AAPL".to_string(),
            // FOREX #unsupported
            //"EUR/USD".to_string(),
            // FUTURE #unsupported
            // https://help.streetsmart.schwab.com/edge/1.68/Content/Futures%20Symbols.htm
            // '/' + 'root symbol' + 'month code' + 'year code'
            //"/ESZ24".to_string(),
            // FUTURE_OPTION #unsupported
            //"ESZ24.CME".to_string(),
            // INDEX
            "$SPX".to_string(),
            // MUTUAL_FUND
            "AAAIX".to_string(),
            // OPTION
            // Symbol (max. 6 characters) + Yr (YY) + Mo (MM) + Day (DD) + Call or Put (C/P) + Strike Price (#####.###) listed with five digits before the decimal and three digits following the decimal
            // "AAPL  240517C00100000".into(),
            get_option_chain("AAPL".to_string()).await,
        ];

        let api = client().await;
        for symbol in symbols {
            tracing::debug!(%symbol);
            let req = api.get_quote(symbol).await.unwrap();
            let rsp = req.send().await.unwrap();
            tracing::debug!(?rsp);
        }
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_option_chains() {
        let api = client().await;
        let mut req = api.get_option_chains("AAPL".into()).await.unwrap();
        req.days_to_expiration(3)
            .exp_month(parameter::Month::All)
            .contract_type(parameter::ContractType::All);
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    async fn get_option_chain(symbol: String) -> String {
        let api = client().await;
        let req = api.get_option_chains(symbol).await.unwrap();
        let rsp = req.send().await.unwrap();
        if let Some(v) = rsp.call_exp_date_map.into_values().next()
            && let Some(mut v) = v.into_values().next()
        {
            return v.pop().expect("must exist").symbol;
        }

        unreachable!()
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_option_expiration_chain() {
        let api = client().await;
        let req = api
            .get_option_expiration_chain("AAPL".into())
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_price_history() {
        let api = client().await;
        let req = api.get_price_history("AAPL".into()).await.unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_movers() {
        let api = client().await;
        let req = api.get_movers("$DJI".into()).await.unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_markets() {
        let api = client().await;
        let req = api
            .get_markets(vec![Market::Equity, Market::Option])
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_market() {
        let api = client().await;
        let req = api.get_market(Market::Equity).await.unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_instruments() {
        let api = client().await;
        let req = api
            .get_instruments("VTI".into(), Projection::SymbolSearch)
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);

        let req = api
            .get_instruments("AAPL".into(), Projection::Fundamental)
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);

        let req = api
            .get_instruments("SNOW".into(), Projection::Fundamental)
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_instrument() {
        let api = client().await;
        let req = api.get_instrument("922908769".into()).await.unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_account_numbers() {
        let api = client().await;
        let req = api.get_account_numbers().await.unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    async fn account_number() -> String {
        let api = client().await;
        let req = api.get_account_numbers().await.unwrap();
        let rsp = req.send().await.unwrap();
        rsp[0].hash_value.clone()
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_accounts() {
        let api = client().await;
        let req = api.get_accounts().await.unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_account() {
        let api = client().await;
        let req = api.get_account(account_number().await).await.unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_account_orders() {
        let api = client().await;
        let req = api
            .get_account_orders(
                account_number().await,
                chrono::NaiveDate::from_ymd_opt(2024, 1, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
                chrono::NaiveDate::from_ymd_opt(2025, 1, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
            )
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    async fn get_account_orders() -> i64 {
        let api = client().await;
        let req = api
            .get_account_orders(
                account_number().await,
                chrono::NaiveDate::from_ymd_opt(2024, 5, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
                chrono::NaiveDate::from_ymd_opt(2024, 5, 26)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
            )
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        rsp[0].order_id
    }

    #[cfg_attr(
        not(all(feature = "test_online", feature = "danger")),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[allow(clippy::too_many_lines)]
    #[test(tokio::test)]
    async fn test_post_put_delete_account_order() {
        let api = client().await;

        let symbol = InstrumentRequest::Equity {
            symbol: "VEA".to_string(),
        };
        let quantity = 1.0;
        let price = 10.0;
        let modified_price = 11.0;

        // preview
        let order_preview =
            model::OrderRequest::limit(symbol.clone(), Instruction::Buy, quantity, price).unwrap();
        let req = api
            .post_accounts_preview_order(account_number().await, order_preview)
            .await
            .unwrap();
        let preview = req.send().await.unwrap();
        tracing::debug!(?preview);

        // post
        let order_post =
            model::OrderRequest::limit(symbol.clone(), Instruction::Buy, quantity, price).unwrap();
        let req = api
            .post_account_order(account_number().await, order_post.clone())
            .await
            .unwrap();
        let order_id = req.send().await.unwrap().unwrap();

        // post check
        let req = api
            .get_account_order(account_number().await, order_id)
            .await
            .unwrap();
        let order_post_check = req.send().await.unwrap();
        tracing::debug!(?order_post_check);
        assert_eq!(
            order_post_check.session,
            model::trader::order::Session::Normal
        );
        assert_approx_eq!(f64, order_post_check.price, price);
        assert_eq!(
            order_post_check.duration,
            model::trader::order::Duration::Day
        );
        assert_eq!(
            order_post_check.order_type,
            model::trader::order::OrderType::Limit
        );
        assert_eq!(
            Into::<InstrumentRequest>::into(
                order_post_check.order_leg_collection[0].instrument.clone()
            ),
            symbol
        );
        assert_eq!(
            Into::<Instruction>::into(order_post_check.order_leg_collection[0].instruction),
            Instruction::Buy
        );
        assert_approx_eq!(
            f64,
            order_post_check.order_leg_collection[0].quantity,
            quantity
        );

        // put
        let order_id = order_post_check.order_id;
        let mut order_put: model::OrderRequest = order_post_check.into();
        order_put.price = Some(modified_price);
        let req = api
            .put_account_order(account_number().await, order_id, order_put.clone())
            .await
            .unwrap();
        let order_id = req.send().await.unwrap().unwrap();

        // put check
        let req = api
            .get_account_order(account_number().await, order_id)
            .await
            .unwrap();
        let order_put_check = req.send().await.unwrap();
        tracing::debug!(?order_put_check);
        assert_eq!(
            order_put_check.session,
            model::trader::order::Session::Normal
        );
        assert_approx_eq!(f64, order_put_check.price, modified_price);
        assert_eq!(
            order_put_check.duration,
            model::trader::order::Duration::Day
        );
        assert_eq!(
            order_put_check.order_type,
            model::trader::order::OrderType::Limit
        );
        assert_eq!(
            Into::<InstrumentRequest>::into(
                order_put_check.order_leg_collection[0].instrument.clone()
            ),
            symbol
        );
        assert_eq!(
            Into::<Instruction>::into(order_put_check.order_leg_collection[0].instruction),
            Instruction::Buy
        );
        assert_approx_eq!(
            f64,
            order_put_check.order_leg_collection[0].quantity,
            quantity
        );

        // delete
        let req = api
            .delete_account_order(account_number().await, order_id)
            .await
            .unwrap();
        req.send().await.unwrap();

        // get check
        let req = api
            .get_account_order(account_number().await, order_id)
            .await
            .unwrap();
        let order = req.send().await.unwrap();
        tracing::debug!(?order);
        assert_eq!(
            order.order_activity_collection.unwrap()[0].execution_type,
            ExecutionType::Canceled
        );
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_account_order() {
        let api = client().await;
        let req = api
            .get_account_order(account_number().await, get_account_orders().await)
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_accounts_orders() {
        let api = client().await;
        let req = api
            .get_accounts_orders(
                chrono::NaiveDate::from_ymd_opt(2024, 6, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
                chrono::NaiveDate::from_ymd_opt(2025, 5, 20)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
            )
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(all(feature = "test_online")),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_post_accounts_preview_order() {
        let api = client().await;
        let req = api
            .post_accounts_preview_order(
                account_number().await,
                model::OrderRequest::limit(
                    InstrumentRequest::Equity {
                        symbol: "VEA".to_string(),
                    },
                    Instruction::Buy,
                    1.0,
                    10.0,
                )
                .unwrap(),
            )
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_account_transactions() {
        // # duplicate field `assetType`
        let api = client().await;
        let req = api
            .get_account_transactions(
                account_number().await,
                chrono::NaiveDate::from_ymd_opt(2023, 5, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
                chrono::NaiveDate::from_ymd_opt(2024, 5, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
                TransactionType::Trade,
            )
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    async fn get_account_transactions() -> i64 {
        // # duplicate field `assetType`

        let api = client().await;
        let req = api
            .get_account_transactions(
                account_number().await,
                chrono::NaiveDate::from_ymd_opt(2023, 5, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
                chrono::NaiveDate::from_ymd_opt(2024, 5, 1)
                    .unwrap()
                    .and_hms_milli_opt(0, 0, 1, 444)
                    .unwrap()
                    .and_local_timezone(chrono::Utc)
                    .unwrap(),
                TransactionType::Trade,
            )
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        rsp[0].activity_id
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_account_transaction() {
        // # duplicate field `assetType`

        let api = client().await;
        let req = api
            .get_account_transaction(account_number().await, get_account_transactions().await)
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_API_KEY, SCHWAB_SECRET and SCHWAB_CALLBACK_URL need to be provided in the environment."#
    )]
    #[test(tokio::test)]
    async fn test_get_user_preference() {
        let api = client().await;
        let req = api.get_user_preference().await.unwrap();
        let rsp = req.send().await.unwrap();
        tracing::debug!(?rsp);
    }

    #[test(tokio::test)]
    async fn test_save_raw_json_creates_formatted_file() {
        let dir = tempdir().expect("Failed to create temp dir");
        let temp_path = dir.path();

        let model_name = "test_model";
        let raw_json = r#"{"id":1,"name":"test","status":"active"}"#;
        let expected_substring = "\"name\": \"test\"";

        save_raw_json(temp_path.to_str().unwrap(), model_name, raw_json);

        let entries = fs::read_dir(temp_path).expect("Should be able to read temp directory");
        let found = entries.flatten().any(|entry| {
            let file_name = entry.file_name().to_string_lossy().into_owned();
            if file_name.contains(model_name)
                && Path::new(&file_name)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            {
                let content = fs::read_to_string(entry.path()).expect("Should read file content");

                assert!(
                    content.contains(expected_substring),
                    "JSON should be pretty-printed"
                );
                assert!(
                    content.contains('\n'),
                    "Pretty-printed JSON should contain newlines"
                );
                return true;
            }
            false
        });

        assert!(found, "JSON file should exist in the temp directory");
    }

    #[test(tokio::test)]
    async fn test_save_raw_json_handles_invalid_json() {
        let dir = tempdir().expect("Failed to create temp dir");
        let temp_path = dir.path();

        let model_name = "invalid_test";
        let invalid_json = "{ not a json }";

        save_raw_json(temp_path.to_str().unwrap(), model_name, invalid_json);

        let entries = fs::read_dir(temp_path).expect("Should read temp directory");
        let found = entries.flatten().any(|entry| {
            let file_name = entry.file_name().to_string_lossy().into_owned();
            if file_name.contains(model_name) {
                let content = fs::read_to_string(entry.path()).unwrap();
                return content == invalid_json;
            }
            false
        });

        assert!(found, "Invalid JSON should still be saved as raw text");
    }
}
