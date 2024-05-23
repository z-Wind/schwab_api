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
pub struct Api<T: Tokener> {
    tokener: T,
    client: Client,
}

impl<T: Tokener> Api<T> {
    /// Create API Struct
    pub fn new(tokener: T) -> Self {
        let client = Client::new();

        Api { tokener, client }
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
    pub async fn get_instruments(
        &self,
        symbol: String,
        projection: Projection,
    ) -> Result<market_data::GetInstrumentsRequest, Error> {
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
    pub async fn get_instrument(
        &self,
        cusip_id: String,
    ) -> Result<market_data::GetInstrumentRequest, Error> {
        let access_token = self.tokener.get_access_token().await?;

        Ok(market_data::GetInstrumentRequest::new(
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

    use crate::model::trader::accounts::AccountCollectiveInvestment;
    use crate::model::trader::accounts::AccountsBaseInstrument;
    use crate::model::trader::accounts::AccountsInstrument;
    use crate::model::trader::preview_order::Instruction;
    use crate::token::TokenChecker;

    async fn client() -> Api<TokenChecker> {
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

        let certs_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/certs"));

        let callback_url = "https://127.0.0.1:8080".to_string();

        let token_checker = TokenChecker::new(path, key, secret, callback_url, certs_dir)
            .await
            .unwrap();

        Api::new(token_checker)
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
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
        dbg!(rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
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
            dbg!(&symbol);
            let req = api.get_quote(symbol).await.unwrap();
            let rsp = req.send().await.unwrap();
            dbg!(rsp);
        }
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_option_chains() {
        let api = client().await;
        let mut req = api.get_option_chains("AAPL".into()).await.unwrap();
        req.days_to_expiration(3)
            .exp_month(parameter::Month::All)
            .contract_type(parameter::ContractType::All);
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    async fn get_option_chain(symbol: String) -> String {
        let api = client().await;
        let req = api.get_option_chains(symbol).await.unwrap();
        let rsp = req.send().await.unwrap();
        if let Some(v) = rsp.call_exp_date_map.into_values().next() {
            if let Some(mut v) = v.into_values().next() {
                return v.pop().expect("must exist").symbol;
            }
        }
        unreachable!()
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_option_expiration_chain() {
        let api = client().await;
        let req = api
            .get_option_expiration_chain("AAPL".into())
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_price_history() {
        let api = client().await;
        let req = api.get_price_history("AAPL".into()).await.unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_movers() {
        let api = client().await;
        let req = api.get_movers("$DJI".into()).await.unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_markets() {
        let api = client().await;
        let req = api
            .get_markets(vec![Market::Equity, Market::Option])
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_market() {
        let api = client().await;
        let req = api.get_market(Market::Equity).await.unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_instruments() {
        let api = client().await;
        let req = api
            .get_instruments("VTI".into(), Projection::SymbolSearch)
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_instrument() {
        let api = client().await;
        let req = api.get_instrument("922908769".into()).await.unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_account_numbers() {
        let api = client().await;
        let req = api.get_account_numbers().await.unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    async fn account_number() -> String {
        let api = client().await;
        let req = api.get_account_numbers().await.unwrap();
        let rsp = req.send().await.unwrap();
        rsp[0].hash_value.clone()
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_accounts() {
        let api = client().await;
        let req = api.get_accounts().await.unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_account() {
        let api = client().await;
        let req = api.get_account(account_number().await).await.unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_account_orders() {
        let api = client().await;
        let req = api
            .get_account_orders(
                account_number().await,
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
        dbg!(rsp);
    }

    async fn get_account_orders() -> i64 {
        let api = client().await;
        let req = api
            .get_account_orders(
                account_number().await,
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
        rsp[0].order_id
    }

    #[cfg_attr(
        not(all(feature = "test_online", feature = "danger")),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_post_put_delete_account_order() {
        todo!();
        let api = client().await;

        // #! AccountsInstrument is not Instrument like assetType
        // Instrument
        // {
        // "instruments": [
        // {
        // "cusip": "922908769",
        // "symbol": "VTI",
        // "description": "VANGUARD TOTAL STOCK MARKET ETF",
        // "exchange": "NYSE Arca",
        // "assetType": "ETF"
        // }
        // ]
        // }
        // AccountsInstrument
        // "instrument": {
        // "assetType": "COLLECTIVE_INVESTMENT",
        // "cusip": "922908769",
        // "symbol": "VTI",
        // "description": "VANGUARD TOTAL STOCK MARKET ETF",
        // "instrumentId": 5215623,
        // "type": "EXCHANGE_TRADED_FUND"
        // },
        // get symbol VTI
        // let req = api.get_instrument("922908769".into()).await.unwrap();
        // let symbol = req.send().await.unwrap();
        // assert_eq!(symbol.symbol, "VTI");
        let symbol = AccountsInstrument::CollectiveInvestment(AccountCollectiveInvestment {
            accounts_base_instrument: AccountsBaseInstrument {
                symbol: "VTI".to_string(),
                ..AccountsBaseInstrument::default()
            },
            ..AccountCollectiveInvestment::default()
        });

        // post
        let order_post = model::OrderRequest::limit(symbol, Instruction::Buy, 1.0, 0.0).unwrap();
        let req = api
            .post_account_order(account_number().await, order_post.clone())
            .await
            .unwrap();
        req.send().await.unwrap();

        // post check
        let req = api
            .get_account_orders(
                account_number().await,
                chrono::Local::now()
                    .checked_sub_days(chrono::Days::new(1))
                    .unwrap()
                    .to_utc(),
                chrono::Local::now()
                    .checked_add_days(chrono::Days::new(1))
                    .unwrap()
                    .to_utc(),
            )
            .await
            .unwrap();
        let orders = req.send().await.unwrap();
        let mut order_post_check: model::OrderRequest = orders[0].into();
        assert_eq!(order_post, order_post_check);

        // put
        let order_id = order_post_check.order_id.unwrap();
        let mut order_put = order_post_check;
        order_put.price = Some(0.1);
        let req = api
            .put_account_order(account_number().await, order_id, order_put.clone())
            .await
            .unwrap();
        req.send().await.unwrap();

        // put check
        let req = api
            .get_account_order(account_number().await, order_id)
            .await
            .unwrap();
        let order_put_check = req.send().await.unwrap();
        assert_eq!(order_put, order_put_check.into());

        // delete
        let req = api
            .delete_account_order(account_number().await, order_id)
            .await
            .unwrap();
        req.send().await.unwrap();

        // get check
        let req = api
            .get_account_orders(
                account_number().await,
                chrono::Local::now()
                    .checked_sub_days(chrono::Days::new(1))
                    .unwrap()
                    .to_utc(),
                chrono::Local::now()
                    .checked_add_days(chrono::Days::new(1))
                    .unwrap()
                    .to_utc(),
            )
            .await
            .unwrap();
        let orders = req.send().await.unwrap();
        assert!(orders.is_empty());
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_account_order() {
        let api = client().await;
        let req = api
            .get_account_order(account_number().await, get_account_orders().await)
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
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
        dbg!(rsp);
    }

    #[cfg_attr(
        not(all(feature = "test_online", feature = "danger")),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_post_accounts_preview_order() {
        todo!();
        let api = client().await;
        let req = api
            .post_accounts_preview_order(account_number().await, model::PreviewOrder::default())
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
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
        dbg!(rsp);
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
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_account_transaction() {
        // # duplicate field `assetType`

        let api = client().await;
        let req = api
            .get_account_transaction(account_number().await, get_account_transactions().await)
            .await
            .unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }

    #[cfg_attr(
        not(feature = "test_online"),
        ignore = r#"Without the "test_online" feature enabled, to activate it, corresponding SCHWAB_SECRET and SCHWAB_SECRET need to be provided in the environment."#
    )]
    #[tokio::test]
    async fn test_get_user_preference() {
        let api = client().await;
        let req = api.get_user_preference().await.unwrap();
        let rsp = req.send().await.unwrap();
        dbg!(rsp);
    }
}
