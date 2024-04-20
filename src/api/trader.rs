use reqwest::{Client, RequestBuilder, StatusCode};

use super::endpoints;
use crate::api::Error;
use crate::model;

#[derive(Debug)]
pub struct GetAccountNumbersRequest {
    req: RequestBuilder,
}

impl GetAccountNumbersRequest {
    fn endpoint() -> endpoints::EndpointAccount {
        endpoints::EndpointAccount::AccountNumbers
    }

    pub(crate) fn new(client: &Client, access_token: String) -> Self {
        let req = client.get(Self::endpoint().url()).bearer_auth(access_token);
        Self::new_with(req)
    }

    fn new_with(req: RequestBuilder) -> Self {
        Self { req }
    }

    fn build(self) -> RequestBuilder {
        self.req
    }

    pub async fn send(self) -> Result<model::AccountNumbers, Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::ServiceError(error_response));
        }

        rsp.json::<model::AccountNumbers>()
            .await
            .map_err(std::convert::Into::into)
    }
}

#[derive(Debug)]
pub struct GetAccountsRequest {
    req: RequestBuilder,

    // This allows one to determine which fields they want returned. Possible value in this String can be:
    // positions
    // Example:
    // fields=positions
    fields: Option<String>,
}

impl GetAccountsRequest {
    fn endpoint() -> endpoints::EndpointAccount {
        endpoints::EndpointAccount::Accounts
    }

    pub(crate) fn new(client: &Client, access_token: String) -> Self {
        let req = client.get(Self::endpoint().url()).bearer_auth(access_token);
        Self::new_with(req)
    }

    fn new_with(req: RequestBuilder) -> Self {
        Self { req, fields: None }
    }

    pub fn fields(mut self, val: String) -> Self {
        self.fields = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req;
        if let Some(x) = self.fields {
            req = req.query(&[("fields", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<model::Accounts, Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::ServiceError(error_response));
        }

        rsp.json::<model::Accounts>()
            .await
            .map_err(std::convert::Into::into)
    }
}

#[derive(Debug)]
pub struct GetAccountRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    // The encrypted ID of the account
    account_number: String,

    // This allows one to determine which fields they want returned. Possible value in this String can be:
    // positions
    // Example:
    // fields=positions
    fields: Option<String>,
}

impl GetAccountRequest {
    fn endpoint(account_number: String) -> endpoints::EndpointAccount {
        endpoints::EndpointAccount::Account { account_number }
    }

    pub(crate) fn new(client: &Client, access_token: String, account_number: String) -> Self {
        let req = client
            .get(Self::endpoint(account_number.clone()).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number)
    }

    fn new_with(req: RequestBuilder, account_number: String) -> Self {
        Self {
            req,
            account_number,
            fields: None,
        }
    }

    pub fn fields(mut self, val: String) -> Self {
        self.fields = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req;
        if let Some(x) = self.fields {
            req = req.query(&[("fields", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<model::Account, Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::ServiceError(error_response));
        }

        rsp.json::<model::Account>()
            .await
            .map_err(std::convert::Into::into)
    }
}

#[derive(Debug)]
pub struct GetAccountOrdersRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    // The encrypted ID of the account
    account_number: String,

    // The max number of orders to retrieve. Default is 3000.
    max_results: Option<i64>,

    // Specifies that no orders entered before this time should be returned. Valid ISO-8601 formats are :
    // yyyy-MM-dd'T'HH:mm:ss.SSSZ Date must be within 60 days from today's date. 'toEnteredTime' must also be set.
    from_entered_time: chrono::DateTime<chrono::Utc>,

    // Specifies that no orders entered after this time should be returned.Valid ISO-8601 formats are :
    // yyyy-MM-dd'T'HH:mm:ss.SSSZ. 'fromEnteredTime' must also be set.
    to_entered_time: chrono::DateTime<chrono::Utc>,

    // Specifies that only orders of this status should be returned.
    // Available values : AWAITING_PARENT_ORDER, AWAITING_CONDITION, AWAITING_STOP_CONDITION, AWAITING_MANUAL_REVIEW, ACCEPTED, AWAITING_UR_OUT, PENDING_ACTIVATION, QUEUED, WORKING, REJECTED, PENDING_CANCEL, CANCELED, PENDING_REPLACE, REPLACED, FILLED, EXPIRED, NEW, AWAITING_RELEASE_TIME, PENDING_ACKNOWLEDGEMENT, PENDING_RECALL, UNKNOWN
    status: Option<String>,
}

impl GetAccountOrdersRequest {
    fn endpoint(account_number: String) -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::OrdersAccount { account_number }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        let req = client
            .get(Self::endpoint(account_number.clone()).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, from_entered_time, to_entered_time)
    }

    fn new_with(
        req: RequestBuilder,
        account_number: String,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            req,
            account_number,
            max_results: None,
            from_entered_time,
            to_entered_time,
            status: None,
        }
    }

    pub fn max_results(mut self, val: i64) -> Self {
        self.max_results = Some(val);
        self
    }

    pub fn status(mut self, val: String) -> Self {
        self.status = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req.query(&[
            ("fromEnteredTime", self.from_entered_time.to_string()),
            ("toEnteredTime", self.to_entered_time.to_string()),
        ]);
        if let Some(x) = self.max_results {
            req = req.query(&[("maxResults", x)]);
        }
        if let Some(x) = self.status {
            req = req.query(&[("status", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<Vec<model::Order>, Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::ServiceError(error_response));
        }

        rsp.json::<Vec<model::Order>>()
            .await
            .map_err(std::convert::Into::into)
    }
}

#[derive(Debug)]
pub struct PostAccountOrderRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    // The encrypted ID of the account
    account_number: String,

    body: model::OrderRequest,
}

impl PostAccountOrderRequest {
    fn endpoint(account_number: String) -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::OrdersAccount { account_number }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        body: model::OrderRequest,
    ) -> Self {
        let req = client
            .post(Self::endpoint(account_number.clone()).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, body)
    }

    fn new_with(req: RequestBuilder, account_number: String, body: model::OrderRequest) -> Self {
        Self {
            req,
            account_number,
            body,
        }
    }

    fn build(self) -> RequestBuilder {
        self.req.json(&self.body)
    }

    pub async fn send(self) -> Result<(), Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::CREATED {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::ServiceError(error_response));
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct GetAccountOrderRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    // The encrypted ID of the account
    account_number: String,

    #[allow(dead_code)]
    // The ID of the order being retrieved.
    order_id: i64,
}

impl GetAccountOrderRequest {
    fn endpoint(account_number: String, order_id: i64) -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::Order {
            account_number,
            order_id,
        }
    }
    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        order_id: i64,
    ) -> Self {
        let req = client
            .get(Self::endpoint(account_number.clone(), order_id).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, order_id)
    }

    fn new_with(req: RequestBuilder, account_number: String, order_id: i64) -> Self {
        Self {
            req,
            account_number,
            order_id,
        }
    }

    fn build(self) -> RequestBuilder {
        self.req
    }

    pub async fn send(self) -> Result<model::Order, Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::ServiceError(error_response));
        }

        rsp.json::<model::Order>()
            .await
            .map_err(std::convert::Into::into)
    }
}

#[derive(Debug)]
pub struct DeleteAccountOrderRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    // The encrypted ID of the account
    account_number: String,

    #[allow(dead_code)]
    // The ID of the order being retrieved.
    order_id: i64,
}

impl DeleteAccountOrderRequest {
    fn endpoint(account_number: String, order_id: i64) -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::Order {
            account_number,
            order_id,
        }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        order_id: i64,
    ) -> Self {
        let req = client
            .delete(Self::endpoint(account_number.clone(), order_id).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, order_id)
    }

    fn new_with(req: RequestBuilder, account_number: String, order_id: i64) -> Self {
        Self {
            req,
            account_number,
            order_id,
        }
    }

    fn build(self) -> RequestBuilder {
        self.req
    }

    pub async fn send(self) -> Result<(), Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::ServiceError(error_response));
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct PutAccountOrderRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    // The encrypted ID of the account
    account_number: String,

    #[allow(dead_code)]
    // The ID of the order being retrieved.
    order_id: i64,

    body: model::OrderRequest,
}

impl PutAccountOrderRequest {
    fn endpoint(account_number: String, order_id: i64) -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::Order {
            account_number,
            order_id,
        }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        order_id: i64,
        body: model::OrderRequest,
    ) -> Self {
        let req = client
            .put(Self::endpoint(account_number.clone(), order_id).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, order_id, body)
    }

    fn new_with(
        req: RequestBuilder,
        account_number: String,
        order_id: i64,
        body: model::OrderRequest,
    ) -> Self {
        Self {
            req,
            account_number,
            order_id,
            body,
        }
    }

    fn build(self) -> RequestBuilder {
        self.req.json(&self.body)
    }

    pub async fn send(self) -> Result<(), Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::CREATED {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::ServiceError(error_response));
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct GetAccountsOrdersRequest {
    req: RequestBuilder,

    // The max number of orders to retrieve. Default is 3000.
    max_results: Option<i64>,

    // Specifies that no orders entered before this time should be returned. Valid ISO-8601 formats are- yyyy-MM-dd'T'HH:mm:ss.SSSZ Date must be within 60 days from today's date. 'toEnteredTime' must also be set.
    from_entered_time: chrono::DateTime<chrono::Utc>,

    // Specifies that no orders entered after this time should be returned.Valid ISO-8601 formats are - yyyy-MM-dd'T'HH:mm:ss.SSSZ. 'fromEnteredTime' must also be set.
    to_entered_time: chrono::DateTime<chrono::Utc>,

    // Specifies that only orders of this status should be returned.
    // Available values : AWAITING_PARENT_ORDER, AWAITING_CONDITION, AWAITING_STOP_CONDITION, AWAITING_MANUAL_REVIEW, ACCEPTED, AWAITING_UR_OUT, PENDING_ACTIVATION, QUEUED, WORKING, REJECTED, PENDING_CANCEL, CANCELED, PENDING_REPLACE, REPLACED, FILLED, EXPIRED, NEW, AWAITING_RELEASE_TIME, PENDING_ACKNOWLEDGEMENT, PENDING_RECALL, UNKNOWN
    status: Option<String>,
}

impl GetAccountsOrdersRequest {
    fn endpoint() -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::Orders
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        let req = client.get(Self::endpoint().url()).bearer_auth(access_token);
        Self::new_with(req, from_entered_time, to_entered_time)
    }

    fn new_with(
        req: RequestBuilder,
        from_entered_time: chrono::DateTime<chrono::Utc>,
        to_entered_time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            req,
            max_results: None,
            from_entered_time,
            to_entered_time,
            status: None,
        }
    }

    pub fn max_results(mut self, val: i64) -> Self {
        self.max_results = Some(val);
        self
    }

    pub fn status(mut self, val: String) -> Self {
        self.status = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req.query(&[
            ("fromEnteredTime", self.from_entered_time.to_string()),
            ("toEnteredTime", self.to_entered_time.to_string()),
        ]);
        if let Some(x) = self.max_results {
            req = req.query(&[("maxResults", x)]);
        }
        if let Some(x) = self.status {
            req = req.query(&[("status", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<Vec<model::Order>, Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::ServiceError(error_response));
        }

        rsp.json::<Vec<model::Order>>()
            .await
            .map_err(std::convert::Into::into)
    }
}

#[derive(Debug)]
pub struct PostAccountPreviewOrderRequest {
    req: RequestBuilder,

    #[allow(dead_code)]
    // The encrypted ID of the account
    account_number: String,

    body: model::PreviewOrder,
}

impl PostAccountPreviewOrderRequest {
    fn endpoint(account_number: String) -> endpoints::EndpointOrder {
        endpoints::EndpointOrder::PreviewOrderAccount { account_number }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        body: model::PreviewOrder,
    ) -> Self {
        let req = client
            .post(Self::endpoint(account_number.clone()).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, body)
    }

    fn new_with(req: RequestBuilder, account_number: String, body: model::PreviewOrder) -> Self {
        Self {
            req,
            account_number,
            body,
        }
    }

    fn build(self) -> RequestBuilder {
        self.req.json(&self.body)
    }

    pub async fn send(self) -> Result<model::PreviewOrder, Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::ServiceError(error_response));
        }

        rsp.json::<model::PreviewOrder>()
            .await
            .map_err(std::convert::Into::into)
    }
}

#[derive(Debug)]
pub struct GetAccountTransactions {
    req: RequestBuilder,

    #[allow(dead_code)]
    // The encrypted ID of the account
    account_number: String,

    // Specifies that no transactions entered before this time should be returned. Valid ISO-8601 formats are :
    // yyyy-MM-dd'T'HH:mm:ss.SSSZ Date must be within 60 days from today's date. 'endDate' must also be set.
    start_date: chrono::DateTime<chrono::Utc>,

    // Specifies that no transactions entered after this time should be returned.Valid ISO-8601 formats are :
    // yyyy-MM-dd'T'HH:mm:ss.SSSZ. 'startDate' must also be set.
    end_date: chrono::DateTime<chrono::Utc>,

    // It filters all the transaction activities based on the symbol specified. NOTE: If there is any special character in the symbol, please send th encoded value.
    symbol: Option<String>,

    // Specifies that only transactions of this status should be returned.
    // Available values : TRADE, RECEIVE_AND_DELIVER, DIVIDEND_OR_INTEREST, ACH_RECEIPT, ACH_DISBURSEMENT, CASH_RECEIPT, CASH_DISBURSEMENT, ELECTRONIC_FUND, WIRE_OUT, WIRE_IN, JOURNAL, MEMORANDUM, MARGIN_CALL, MONEY_MARKET, SMA_ADJUSTMENT
    types: String,
}

impl GetAccountTransactions {
    fn endpoint(account_number: String) -> endpoints::EndpointTransaction {
        endpoints::EndpointTransaction::TransactionsAccount { account_number }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        types: String,
    ) -> Self {
        let req = client
            .get(Self::endpoint(account_number.clone()).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, start_date, end_date, types)
    }

    fn new_with(
        req: RequestBuilder,
        account_number: String,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        types: String,
    ) -> Self {
        Self {
            req,
            account_number,
            start_date,
            end_date,
            symbol: None,
            types,
        }
    }

    pub fn symbol(mut self, val: String) -> Self {
        self.symbol = Some(val);
        self
    }

    fn build(self) -> RequestBuilder {
        let mut req = self.req.query(&[
            ("startDate", self.start_date.to_string()),
            ("endDate", self.end_date.to_string()),
            ("types", self.types.to_string()),
        ]);
        if let Some(x) = self.symbol {
            req = req.query(&[("symbol", x)]);
        }

        req
    }

    pub async fn send(self) -> Result<Vec<model::Transaction>, Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::ServiceError(error_response));
        }

        rsp.json::<Vec<model::Transaction>>()
            .await
            .map_err(std::convert::Into::into)
    }
}

#[derive(Debug)]
pub struct GetAccountTransaction {
    req: RequestBuilder,

    #[allow(dead_code)]
    // The encrypted ID of the account
    account_number: String,

    #[allow(dead_code)]
    // The ID of the transaction being retrieved.
    transaction_id: i64,
}

impl GetAccountTransaction {
    fn endpoint(account_number: String, transaction_id: i64) -> endpoints::EndpointTransaction {
        endpoints::EndpointTransaction::Transaction {
            account_number,
            transaction_id,
        }
    }

    pub(crate) fn new(
        client: &Client,
        access_token: String,
        account_number: String,
        transaction_id: i64,
    ) -> Self {
        let req = client
            .get(Self::endpoint(account_number.clone(), transaction_id).url())
            .bearer_auth(access_token);
        Self::new_with(req, account_number, transaction_id)
    }

    fn new_with(req: RequestBuilder, account_number: String, transaction_id: i64) -> Self {
        Self {
            req,
            account_number,
            transaction_id,
        }
    }

    fn build(self) -> RequestBuilder {
        self.req
    }

    pub async fn send(self) -> Result<model::Transaction, Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::ServiceError(error_response));
        }

        let mut transactions = rsp.json::<Vec<model::Transaction>>().await?;
        Ok(transactions.pop().expect("must exist"))
    }
}

#[derive(Debug)]
pub struct GetUserPreferenceRequest {
    req: RequestBuilder,
}

impl GetUserPreferenceRequest {
    fn endpoint() -> endpoints::EndpointUserPreference {
        endpoints::EndpointUserPreference::UserPreference
    }
    pub(crate) fn new(client: &Client, access_token: String) -> Self {
        let req = client.get(Self::endpoint().url()).bearer_auth(access_token);
        Self::new_with(req)
    }

    fn new_with(req: RequestBuilder) -> Self {
        Self { req }
    }

    fn build(self) -> RequestBuilder {
        self.req
    }

    pub async fn send(self) -> Result<Vec<model::UserPreference>, Error> {
        let req = self.build();
        let rsp = req.send().await?;

        let status = rsp.status();
        if status != StatusCode::OK {
            let error_response = rsp.json::<model::ServiceError>().await?;
            return Err(Error::ServiceError(error_response));
        }

        rsp.json::<Vec<model::UserPreference>>()
            .await
            .map_err(std::convert::Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use mockito::Matcher;
    use pretty_assertions::assert_eq;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_account_numbers_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        // none

        // Create a mock
        let mock = server
            .mock("GET", "/accounts/accountNumbers")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/AccountNumbers.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountNumbersRequest::endpoint().url_endpoint()
        ));

        let req = GetAccountNumbersRequest::new_with(req);

        // check initial value
        // none

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result[0].account_number, "string");
    }

    #[tokio::test]
    async fn test_get_accounts_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let fields = "positions".to_string();

        // Create a mock
        let mock = server
            .mock("GET", "/accounts")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "fields".into(),
                fields.to_string(),
            )]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/Accounts.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountsRequest::endpoint().url_endpoint()
        ));

        let mut req = GetAccountsRequest::new_with(req);

        // check initial value
        assert_eq!(req.fields, None);

        // check setter
        req = req.fields(fields.clone());
        assert_eq!(req.fields, Some(fields));

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result[0].securities_account.type_field, "CASH");
    }

    #[tokio::test]
    async fn test_get_account_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let fields = "positions".to_string();

        // Create a mock
        let mock = server
            .mock("GET", "/accounts/account_number")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "fields".into(),
                fields.to_string(),
            )]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/Account.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountRequest::endpoint(account_number.clone()).url_endpoint()
        ));

        let mut req = GetAccountRequest::new_with(req, account_number.clone());

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.fields, None);

        // check setter
        req = req.fields(fields.clone());
        assert_eq!(req.fields, Some(fields));

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.securities_account.type_field, "CASH");
    }

    #[tokio::test]
    async fn test_get_account_orders_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let max_results = 10;
        let from_entered_time = chrono::NaiveDate::from_ymd_opt(2015, 1, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 1, 444)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap();
        let to_entered_time = chrono::NaiveDate::from_ymd_opt(2015, 1, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 1, 444)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap();
        let status = "AWAITING_PARENT_ORDER".to_string();

        // Create a mock
        let mock = server
            .mock("GET", "/accounts/account_number/orders")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("maxResults".into(), max_results.to_string()),
                Matcher::UrlEncoded("fromEnteredTime".into(), from_entered_time.to_string()),
                Matcher::UrlEncoded("toEnteredTime".into(), to_entered_time.to_string()),
                Matcher::UrlEncoded("status".into(), status.to_string()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/Orders.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountOrdersRequest::endpoint(account_number.clone()).url_endpoint()
        ));

        let mut req = GetAccountOrdersRequest::new_with(
            req,
            account_number.clone(),
            from_entered_time,
            to_entered_time,
        );

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.max_results, None);
        assert_eq!(req.from_entered_time, from_entered_time);
        assert_eq!(req.to_entered_time, to_entered_time);
        assert_eq!(req.status, None);

        // check setter
        req = req.max_results(max_results);
        assert_eq!(req.max_results, Some(max_results));
        req = req.status(status.clone());
        assert_eq!(req.status, Some(status));

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_post_account_order_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let body = model::OrderRequest::default();

        // Create a mock
        let mock = server
            .mock("POST", "/accounts/account_number/orders")
            .with_status(201)
            .with_header("content-type", "application/json")
            .match_body(mockito::Matcher::Json(
                serde_json::to_value(body.clone()).unwrap(),
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.post(format!(
            "{url}{}",
            PostAccountOrderRequest::endpoint(account_number.clone()).url_endpoint()
        ));

        let req = PostAccountOrderRequest::new_with(req, account_number.clone(), body.clone());

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.body, body);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_account_order_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let order_id = 123;

        // Create a mock
        let mock = server
            .mock("GET", "/accounts/account_number/orders/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/Order.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountOrderRequest::endpoint(account_number.clone(), order_id).url_endpoint()
        ));

        let req = GetAccountOrderRequest::new_with(req, account_number.clone(), order_id);

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.order_id, order_id);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.session, "NORMAL");
    }

    #[tokio::test]
    async fn test_delete_account_order_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let order_id = 123;

        // Create a mock
        let mock = server
            .mock("DELETE", "/accounts/account_number/orders/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .create_async()
            .await;

        let client = Client::new();
        let req = client.delete(format!(
            "{url}{}",
            DeleteAccountOrderRequest::endpoint(account_number.clone(), order_id).url_endpoint()
        ));

        let req = DeleteAccountOrderRequest::new_with(req, account_number.clone(), order_id);

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.order_id, order_id);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_account_order_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let order_id = 123;
        let body = model::OrderRequest::default();

        // Create a mock
        let mock = server
            .mock("PUT", "/accounts/account_number/orders/123")
            .with_status(201)
            .with_header("content-type", "application/json")
            .match_body(Matcher::Json(serde_json::to_value(body.clone()).unwrap()))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.put(format!(
            "{url}{}",
            PutAccountOrderRequest::endpoint(account_number.clone(), order_id).url_endpoint()
        ));

        let req =
            PutAccountOrderRequest::new_with(req, account_number.clone(), order_id, body.clone());

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.order_id, order_id);
        assert_eq!(req.body, body);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_accounts_orders_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let max_results = 10;
        let from_entered_time = chrono::NaiveDate::from_ymd_opt(2015, 1, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 1, 444)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap();
        let to_entered_time = chrono::NaiveDate::from_ymd_opt(2015, 1, 1)
            .unwrap()
            .and_hms_milli_opt(0, 0, 1, 444)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap();
        let status = "AWAITING_PARENT_ORDER".to_string();

        // Create a mock
        let mock = server
            .mock("GET", "/orders")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("maxResults".into(), max_results.to_string()),
                Matcher::UrlEncoded("fromEnteredTime".into(), from_entered_time.to_string()),
                Matcher::UrlEncoded("toEnteredTime".into(), to_entered_time.to_string()),
                Matcher::UrlEncoded("status".into(), status.to_string()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/Orders.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountsOrdersRequest::endpoint().url_endpoint()
        ));

        let mut req = GetAccountsOrdersRequest::new_with(req, from_entered_time, to_entered_time);

        // check initial value
        assert_eq!(req.max_results, None);
        assert_eq!(req.from_entered_time, from_entered_time);
        assert_eq!(req.to_entered_time, to_entered_time);
        assert_eq!(req.status, None);

        // check setter
        req = req.max_results(max_results);
        assert_eq!(req.max_results, Some(max_results));
        req = req.status(status.clone());
        assert_eq!(req.status, Some(status));

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_post_account_preview_order_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let body = model::PreviewOrder::default();

        // Create a mock
        let mock = server
            .mock("POST", "/accounts/account_number/previewOrder")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/PreviewOrder.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.post(format!(
            "{url}{}",
            PostAccountPreviewOrderRequest::endpoint(account_number.clone()).url_endpoint()
        ));

        let req =
            PostAccountPreviewOrderRequest::new_with(req, account_number.clone(), body.clone());

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.body, body);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.order_id, 0);
    }

    #[tokio::test]
    async fn test_get_account_transactions_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
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
        let symbol = "VTI".to_string();
        let types = "TRADE".to_string();

        // Create a mock
        let mock = server
            .mock("GET", "/accounts/account_number/transactions")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("startDate".into(), start_date.to_string()),
                Matcher::UrlEncoded("endDate".into(), end_date.to_string()),
                Matcher::UrlEncoded("symbol".into(), symbol.to_string()),
                Matcher::UrlEncoded("types".into(), types.to_string()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/Transactions.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountTransactions::endpoint(account_number.clone()).url_endpoint()
        ));

        let mut req = GetAccountTransactions::new_with(
            req,
            account_number.clone(),
            start_date,
            end_date,
            types.clone(),
        );

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.start_date, start_date);
        assert_eq!(req.end_date, end_date);
        assert_eq!(req.symbol, None);
        assert_eq!(req.types, types);

        // check setter
        req = req.symbol(symbol.clone());
        assert_eq!(req.symbol, Some(symbol));

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_get_account_transaction_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        let account_number = "account_number".to_string();
        let transaction_id = 123;

        // Create a mock
        let mock = server
            .mock("GET", "/accounts/account_number/transactions/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"[
                {
                  "activityId": 0,
                  "time": "2024-04-20T03:47:13.958Z",
                  "user": {
                    "cdDomainId": "string",
                    "login": "string",
                    "type": "ADVISOR_USER",
                    "userId": 0,
                    "systemUserName": "string",
                    "firstName": "string",
                    "lastName": "string",
                    "brokerRepCode": "string"
                  },
                  "description": "string",
                  "accountNumber": "string",
                  "type": "TRADE",
                  "status": "VALID",
                  "subAccount": "CASH",
                  "tradeDate": "2024-04-20T03:47:13.958Z",
                  "settlementDate": "2024-04-20T03:47:13.958Z",
                  "positionId": 0,
                  "orderId": 0,
                  "netAmount": 0,
                  "activityType": "ACTIVITY_CORRECTION",
                  "transferItems": [
                    {
                      "instrument": {
                        "cusip": "string",
                        "symbol": "string",
                        "description": "string",
                        "instrumentId": 0,
                        "netChange": 0,
                        "type": "SWEEP_VEHICLE"
                      },
                      "amount": 0,
                      "cost": 0,
                      "price": 0,
                      "feeType": "COMMISSION",
                      "positionEffect": "OPENING"
                    }
                  ]
                }
              ]"#,
            )
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetAccountTransaction::endpoint(account_number.clone(), transaction_id).url_endpoint()
        ));

        let req = GetAccountTransaction::new_with(req, account_number.clone(), transaction_id);

        // check initial value
        assert_eq!(req.account_number, account_number);
        assert_eq!(req.transaction_id, transaction_id);

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.activity_id, 0);
    }

    #[tokio::test]
    async fn test_get_user_preference_request() {
        // Request a new server from the pool
        let mut server = mockito::Server::new_async().await;

        // Use one of these addresses to configure your client
        let _host = server.host_with_port();
        let url = server.url();

        // define parameter
        // none

        // Create a mock
        let mock = server
            .mock("GET", "/userPreference")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/model/Trader/UserPreferences.json"
            ))
            .create_async()
            .await;

        let client = Client::new();
        let req = client.get(format!(
            "{url}{}",
            GetUserPreferenceRequest::endpoint().url_endpoint()
        ));

        let req = GetUserPreferenceRequest::new_with(req);

        // check initial value
        // none

        // check setter
        // none

        dbg!(&req);
        let result = req.send().await;
        mock.assert_async().await;
        let result = result.unwrap();
        assert_eq!(result.len(), 1);
    }
}
