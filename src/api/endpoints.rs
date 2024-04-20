/// specifies Endpoints for Schwab API

const SERVER_TRADER: &str = "https://api.schwabapi.com/trader/v1";
const SERVER_MARKETDATA: &str = "https://api.schwabapi.com/marketdata/v1";

#[derive(Debug)]
pub(crate) enum EndpointAccount {
    // GET
    // /accounts/accountNumbers
    // Get list of account numbers and their encrypted values
    AccountNumbers,

    // GET
    // /accounts
    // Get linked account(s) balances and positions for the logged in user.
    Accounts,

    // GET
    // /accounts/{accountNumber}
    // Get a specific account balance and positions for the logged in user.
    Account { account_number: String },
}

impl EndpointAccount {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointAccount::AccountNumbers => "/accounts/accountNumbers".to_string(),
            EndpointAccount::Accounts => "/accounts".to_string(),
            EndpointAccount::Account { account_number } => {
                format!("/accounts/{account_number}")
            }
        }
    }

    /// defines the URL include server
    pub(crate) fn url(&self) -> String {
        format!("{SERVER_TRADER}{}", self.url_endpoint())
    }
}

#[derive(Debug)]
pub(crate) enum EndpointOrder {
    // GET
    // /accounts/{accountNumber}/orders
    // Get all orders for a specific account.
    // POST
    // /accounts/{accountNumber}/orders
    // Place order for a specific account.
    OrdersAccount {
        account_number: String,
    },

    // GET
    // /accounts/{accountNumber}/orders/{orderId}
    // Get a specific order by its ID, for a specific account
    // DELETE
    // /accounts/{accountNumber}/orders/{orderId}
    // Cancel an order for a specific account
    // PUT
    // /accounts/{accountNumber}/orders/{orderId}
    // Replace order for a specific account
    Order {
        account_number: String,
        order_id: i64,
    },

    // GET
    // /orders
    // Get all orders for all accounts
    Orders,

    // POST
    // /accounts/{accountNumber}/previewOrder
    // Preview order for a specific account. **Coming Soon**.
    PreviewOrderAccount {
        account_number: String,
    },
}

impl EndpointOrder {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointOrder::OrdersAccount { account_number } => {
                format!("/accounts/{account_number}/orders")
            }
            EndpointOrder::Order {
                account_number,
                order_id,
            } => format!("/accounts/{account_number}/orders/{order_id}"),
            EndpointOrder::Orders => "/orders".to_string(),
            EndpointOrder::PreviewOrderAccount { account_number } => {
                format!("/accounts/{account_number}/previewOrder")
            }
        }
    }

    /// defines the URL include server
    pub(crate) fn url(&self) -> String {
        format!("{SERVER_TRADER}{}", self.url_endpoint())
    }
}

#[derive(Debug)]
pub(crate) enum EndpointTransaction {
    // GET
    // /accounts/{accountNumber}/transactions
    // Get all transactions information for a specific account.
    TransactionsAccount {
        account_number: String,
    },

    // GET
    // /accounts/{accountNumber}/transactions/{transactionId}
    // Get specific transaction information for a specific account
    Transaction {
        account_number: String,
        transaction_id: i64,
    },
}

impl EndpointTransaction {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointTransaction::TransactionsAccount { account_number } => {
                format!("/accounts/{account_number}/transactions")
            }
            EndpointTransaction::Transaction {
                account_number,
                transaction_id,
            } => {
                format!("/accounts/{account_number}/transactions/{transaction_id}")
            }
        }
    }

    /// defines the URL include server
    pub(crate) fn url(&self) -> String {
        format!("{SERVER_TRADER}{}", self.url_endpoint())
    }
}

#[derive(Debug)]
pub(crate) enum EndpointUserPreference {
    // GET
    // /userPreference
    // Get user preference information for the logged in user.
    UserPreference,
}

impl EndpointUserPreference {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointUserPreference::UserPreference => "/userPreference".to_string(),
        }
    }

    /// defines the URL include server
    pub(crate) fn url(&self) -> String {
        format!("{SERVER_TRADER}{}", self.url_endpoint())
    }
}

#[derive(Debug)]
pub(crate) enum EndpointQuote {
    // GET
    // /quotes
    // Get Quotes by list of symbols.
    Quotes,

    // GET
    // /{symbol_id}/quotes
    // Get Quote by single symbol.
    Quote { symbol_id: String },
}

impl EndpointQuote {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointQuote::Quotes => "/quotes".to_string(),
            EndpointQuote::Quote { symbol_id } => {
                format!("/{symbol_id}/quotes")
            }
        }
    }

    /// defines the URL include server
    pub(crate) fn url(&self) -> String {
        format!("{SERVER_MARKETDATA}{}", self.url_endpoint())
    }
}

#[derive(Debug)]
pub(crate) enum EndpointOptionChain {
    // GET
    // /chains
    // Get option chain for an optionable Symbol
    Chains,
}

impl EndpointOptionChain {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointOptionChain::Chains => "/chains".to_string(),
        }
    }

    /// defines the URL include server
    pub(crate) fn url(&self) -> String {
        format!("{SERVER_MARKETDATA}{}", self.url_endpoint())
    }
}

#[derive(Debug)]
pub(crate) enum EndpointOptionExpirationChain {
    // GET
    // /expirationchain
    // Get option expiration chain for an optionable symbol
    ExpirationChain,
}

impl EndpointOptionExpirationChain {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointOptionExpirationChain::ExpirationChain => "/expirationchain".to_string(),
        }
    }

    /// defines the URL include server
    pub(crate) fn url(&self) -> String {
        format!("{SERVER_MARKETDATA}{}", self.url_endpoint())
    }
}

#[derive(Debug)]
pub(crate) enum EndpointPriceHistory {
    // GET
    // /pricehistory
    // Get PriceHistory for a single symbol and date ranges.
    PriceHistory,
}

impl EndpointPriceHistory {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointPriceHistory::PriceHistory => "/pricehistory".to_string(),
        }
    }

    /// defines the URL include server
    pub(crate) fn url(&self) -> String {
        format!("{SERVER_MARKETDATA}{}", self.url_endpoint())
    }
}

#[derive(Debug)]
pub(crate) enum EndpointMover {
    // GET
    // /movers/{symbol_id}
    // Get Movers for a specific index.
    Mover { symbol_id: String },
}

impl EndpointMover {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointMover::Mover { symbol_id } => {
                format!("/movers/{symbol_id}")
            }
        }
    }

    /// defines the URL include server
    pub(crate) fn url(&self) -> String {
        format!("{SERVER_MARKETDATA}{}", self.url_endpoint())
    }
}

#[derive(Debug)]
pub(crate) enum EndpointMarketHour {
    // GET
    // /markets
    // Get Market Hours for different markets.
    Markets,

    // GET
    // /markets/{market_id}
    // Get Market Hours for a single market.
    Market { market_id: String },
}

impl EndpointMarketHour {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointMarketHour::Markets => "/markets".to_string(),
            EndpointMarketHour::Market { market_id } => {
                format!("/markets/{market_id}")
            }
        }
    }

    /// defines the URL include server
    pub(crate) fn url(&self) -> String {
        format!("{SERVER_MARKETDATA}{}", self.url_endpoint())
    }
}

#[derive(Debug)]
pub(crate) enum EndpointInstrument {
    // GET
    // /instruments
    // Get Instruments by symbols and projections.
    Instrutments,

    // GET
    // /instruments/{cusip_id}
    // Get Instrument by specific cusip
    Instrutment { cusip_id: String },
}

impl EndpointInstrument {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointInstrument::Instrutments => "/instrutments".to_string(),
            EndpointInstrument::Instrutment { cusip_id } => {
                format!("/instrutments/{cusip_id}")
            }
        }
    }

    /// defines the URL include server
    pub(crate) fn url(&self) -> String {
        format!("{SERVER_MARKETDATA}{}", self.url_endpoint())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_account() {
        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts/accountNumbers",
            EndpointAccount::AccountNumbers.url()
        );

        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts",
            EndpointAccount::Accounts.url()
        );

        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts/123456",
            EndpointAccount::Account {
                account_number: "123456".to_string()
            }
            .url()
        );
    }

    #[test]
    fn test_endpoint_order() {
        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts/123456/orders",
            EndpointOrder::OrdersAccount {
                account_number: "123456".to_string()
            }
            .url()
        );

        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts/123456/orders/789",
            EndpointOrder::Order {
                account_number: "123456".to_string(),
                order_id: 789
            }
            .url()
        );

        assert_eq!(
            "https://api.schwabapi.com/trader/v1/orders",
            EndpointOrder::Orders.url()
        );

        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts/123456/previewOrder",
            EndpointOrder::PreviewOrderAccount {
                account_number: "123456".to_string()
            }
            .url()
        );
    }

    #[test]
    fn test_endpoint_transaction() {
        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts/123456/transactions",
            EndpointTransaction::TransactionsAccount {
                account_number: "123456".to_string()
            }
            .url()
        );

        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts/123456/transactions/789",
            EndpointTransaction::Transaction {
                account_number: "123456".to_string(),
                transaction_id: 789
            }
            .url()
        );
    }

    #[test]
    fn test_endpoint_user_preference() {
        assert_eq!(
            "https://api.schwabapi.com/trader/v1/userPreference",
            EndpointUserPreference::UserPreference.url()
        );
    }

    #[test]
    fn test_endpoint_quote() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/quotes",
            EndpointQuote::Quotes.url()
        );

        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/ABC/quotes",
            EndpointQuote::Quote {
                symbol_id: "ABC".to_string()
            }
            .url()
        );
    }

    #[test]
    fn test_endpoint_option_chain() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/chains",
            EndpointOptionChain::Chains.url()
        );
    }

    #[test]
    fn test_endpoint_option_expiration_chain() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/expirationchain",
            EndpointOptionExpirationChain::ExpirationChain.url()
        );
    }

    #[test]
    fn test_endpoint_price_history() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/pricehistory",
            EndpointPriceHistory::PriceHistory.url()
        );
    }

    #[test]
    fn test_endpoint_mover() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/movers/ABC",
            EndpointMover::Mover {
                symbol_id: "ABC".to_string()
            }
            .url()
        );
    }

    #[test]
    fn test_endpoint_market_hour() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/markets",
            EndpointMarketHour::Markets.url()
        );

        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/markets/XYZ",
            EndpointMarketHour::Market {
                market_id: "XYZ".to_string()
            }
            .url()
        );
    }

    #[test]
    fn test_endpoint_instrument() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/instrutments",
            EndpointInstrument::Instrutments.url()
        );

        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/instrutments/123456",
            EndpointInstrument::Instrutment {
                cusip_id: "123456".to_string()
            }
            .url()
        );
    }
}
