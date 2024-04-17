/// specifies Endpoints for Schwab API

#[derive(Debug)]
pub(crate) enum EndpointAccount<'a> {
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
    Account { account_number: &'a str },
}

impl<'a> EndpointAccount<'a> {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointAccount::AccountNumbers => {
                format!("{ENDPOINT_TRADER}/accounts/accountNumbers")
            }
            EndpointAccount::Accounts => {
                format!("{ENDPOINT_TRADER}/accounts")
            }
            EndpointAccount::Account { account_number } => {
                format!("{ENDPOINT_TRADER}/accounts/{account_number}")
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum EndpointOrder<'a> {
    // GET
    // /accounts/{accountNumber}/orders
    // Get all orders for a specific account.
    // POST
    // /accounts/{accountNumber}/orders
    // Place order for a specific account.
    OrdersAccount {
        account_number: &'a str,
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
        account_number: &'a str,
        order_id: &'a str,
    },

    // GET
    // /orders
    // Get all orders for all accounts
    Orders,

    // POST
    // /accounts/{accountNumber}/previewOrder
    // Preview order for a specific account. **Coming Soon**.
    PreviewOrderAccount {
        account_number: &'a str,
    },
}

impl<'a> EndpointOrder<'a> {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointOrder::OrdersAccount { account_number } => {
                format!("{ENDPOINT_TRADER}/accounts/{account_number}/orders")
            }
            EndpointOrder::Order {
                account_number,
                order_id,
            } => format!("{ENDPOINT_TRADER}/{account_number}/orders/{order_id}"),
            EndpointOrder::Orders => {
                format!("{ENDPOINT_TRADER}/orders")
            }
            EndpointOrder::PreviewOrderAccount { account_number } => {
                format!("{ENDPOINT_TRADER}/accounts/{account_number}/previewOrder")
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum EndpointTransaction<'a> {
    // GET
    // /accounts/{accountNumber}/transactions
    // Get all transactions information for a specific account.
    TransactionsAccount {
        account_number: &'a str,
    },

    // GET
    // /accounts/{accountNumber}/transactions/{transactionId}
    // Get specific transaction information for a specific account
    Transaction {
        account_number: &'a str,
        transaction_id: &'a str,
    },
}

impl<'a> EndpointTransaction<'a> {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointTransaction::TransactionsAccount { account_number } => {
                format!("{ENDPOINT_TRADER}/accounts/{account_number}/transactions")
            }
            EndpointTransaction::Transaction {
                account_number,
                transaction_id,
            } => {
                format!("{ENDPOINT_TRADER}/accounts/{account_number}/transactions/{transaction_id}")
            }
        }
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
            EndpointUserPreference::UserPreference => format!("{ENDPOINT_TRADER}/userPreference"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum EndpointQuote<'a> {
    // GET
    // /quotes
    // Get Quotes by list of symbols.
    Quotes,

    // GET
    // /{symbol_id}/quotes
    // Get Quote by single symbol.
    Quote { symbol_id: &'a str },
}

impl<'a> EndpointQuote<'a> {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointQuote::Quotes => format!("{ENDPOINT_MARKETDATA}/quotes"),
            EndpointQuote::Quote { symbol_id } => {
                format!("{ENDPOINT_MARKETDATA}/{symbol_id}/quotes")
            }
        }
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
            EndpointOptionChain::Chains => format!("{ENDPOINT_MARKETDATA}/chains"),
        }
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
            EndpointOptionExpirationChain::ExpirationChain => {
                format!("{ENDPOINT_MARKETDATA}/expirationchain")
            }
        }
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
            EndpointPriceHistory::PriceHistory => format!("{ENDPOINT_MARKETDATA}/pricehistory"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum EndpointMover<'a> {
    // GET
    // /movers/{symbol_id}
    // Get Movers for a specific index.
    Mover { symbol_id: &'a str },
}

impl<'a> EndpointMover<'a> {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointMover::Mover { symbol_id } => {
                format!("{ENDPOINT_MARKETDATA}/movers/{symbol_id}")
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum EndpointMarketHour<'a> {
    // GET
    // /markets
    // Get Market Hours for different markets.
    Markets,

    // GET
    // /markets/{market_id}
    // Get Market Hours for a single market.
    Market { market_id: &'a str },
}

impl<'a> EndpointMarketHour<'a> {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointMarketHour::Markets => format!("{ENDPOINT_MARKETDATA}/markets"),
            EndpointMarketHour::Market { market_id } => {
                format!("{ENDPOINT_MARKETDATA}/markets/{market_id}")
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum EndpointInstrument<'a> {
    // GET
    // /instruments
    // Get Instruments by symbols and projections.
    Instruments,

    // GET
    // /instruments/{cusip_id}
    // Get Instrument by specific cusip
    Instrument { cusip_id: &'a str },
}

impl<'a> EndpointInstrument<'a> {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            EndpointInstrument::Instruments => format!("{ENDPOINT_MARKETDATA}/instruments"),
            EndpointInstrument::Instrument { cusip_id } => {
                format!("{ENDPOINT_MARKETDATA}/instruments/{cusip_id}")
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum Endpoint<'a> {
    // Trader
    Account(EndpointAccount<'a>),
    Order(EndpointOrder<'a>),
    Transaction(EndpointTransaction<'a>),
    UserPreference(EndpointUserPreference),

    //Market Data
    Quote(EndpointQuote<'a>),
    OptionChain(EndpointOptionChain),
    OptionExpirationChain(EndpointOptionExpirationChain),
    PriceHistory(EndpointPriceHistory),
    Mover(EndpointMover<'a>),
    MarketHour(EndpointMarketHour<'a>),
    Instrument(EndpointInstrument<'a>),
}

const ENDPOINT_TRADER: &str = "https://api.schwabapi.com/trader/v1";
const ENDPOINT_MARKETDATA: &str = "https://api.schwabapi.com/marketdata/v1";

impl<'a> Endpoint<'a> {
    /// defines the URL for the specified Endpoint
    pub(crate) fn url_endpoint(&self) -> String {
        match self {
            Endpoint::Account(endpoint) => endpoint.url_endpoint(),
            Endpoint::Order(endpoint) => endpoint.url_endpoint(),
            Endpoint::Transaction(endpoint) => endpoint.url_endpoint(),
            Endpoint::UserPreference(endpoint) => endpoint.url_endpoint(),
            Endpoint::Quote(endpoint) => endpoint.url_endpoint(),
            Endpoint::OptionChain(endpoint) => endpoint.url_endpoint(),
            Endpoint::OptionExpirationChain(endpoint) => endpoint.url_endpoint(),
            Endpoint::PriceHistory(endpoint) => endpoint.url_endpoint(),
            Endpoint::Mover(endpoint) => endpoint.url_endpoint(),
            Endpoint::MarketHour(endpoint) => endpoint.url_endpoint(),
            Endpoint::Instrument(endpoint) => endpoint.url_endpoint(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_account() {
        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts/accountNumbers",
            Endpoint::Account(EndpointAccount::AccountNumbers).url_endpoint()
        );

        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts",
            Endpoint::Account(EndpointAccount::Accounts).url_endpoint()
        );

        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts/123456",
            Endpoint::Account(EndpointAccount::Account {
                account_number: "123456"
            })
            .url_endpoint()
        );
    }

    #[test]
    fn test_endpoint_order() {
        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts/123456/orders",
            Endpoint::Order(EndpointOrder::OrdersAccount {
                account_number: "123456"
            })
            .url_endpoint()
        );

        assert_eq!(
            "https://api.schwabapi.com/trader/v1/123456/orders/789",
            Endpoint::Order(EndpointOrder::Order {
                account_number: "123456",
                order_id: "789"
            })
            .url_endpoint()
        );

        assert_eq!(
            "https://api.schwabapi.com/trader/v1/orders",
            Endpoint::Order(EndpointOrder::Orders).url_endpoint()
        );

        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts/123456/previewOrder",
            Endpoint::Order(EndpointOrder::PreviewOrderAccount {
                account_number: "123456"
            })
            .url_endpoint()
        );
    }

    #[test]
    fn test_endpoint_transaction() {
        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts/123456/transactions",
            Endpoint::Transaction(EndpointTransaction::TransactionsAccount {
                account_number: "123456"
            })
            .url_endpoint()
        );

        assert_eq!(
            "https://api.schwabapi.com/trader/v1/accounts/123456/transactions/789",
            Endpoint::Transaction(EndpointTransaction::Transaction {
                account_number: "123456",
                transaction_id: "789"
            })
            .url_endpoint()
        );
    }

    #[test]
    fn test_endpoint_user_preference() {
        assert_eq!(
            "https://api.schwabapi.com/trader/v1/userPreference",
            Endpoint::UserPreference(EndpointUserPreference::UserPreference).url_endpoint()
        );
    }

    #[test]
    fn test_endpoint_quote() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/quotes",
            Endpoint::Quote(EndpointQuote::Quotes).url_endpoint()
        );

        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/ABC/quotes",
            Endpoint::Quote(EndpointQuote::Quote { symbol_id: "ABC" }).url_endpoint()
        );
    }

    #[test]
    fn test_endpoint_option_chain() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/chains",
            Endpoint::OptionChain(EndpointOptionChain::Chains).url_endpoint()
        );
    }

    #[test]
    fn test_endpoint_option_expiration_chain() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/expirationchain",
            Endpoint::OptionExpirationChain(EndpointOptionExpirationChain::ExpirationChain)
                .url_endpoint()
        );
    }

    #[test]
    fn test_endpoint_price_history() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/pricehistory",
            Endpoint::PriceHistory(EndpointPriceHistory::PriceHistory).url_endpoint()
        );
    }

    #[test]
    fn test_endpoint_mover() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/movers/ABC",
            Endpoint::Mover(EndpointMover::Mover { symbol_id: "ABC" }).url_endpoint()
        );
    }

    #[test]
    fn test_endpoint_market_hour() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/markets",
            Endpoint::MarketHour(EndpointMarketHour::Markets).url_endpoint()
        );

        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/markets/XYZ",
            Endpoint::MarketHour(EndpointMarketHour::Market { market_id: "XYZ" }).url_endpoint()
        );
    }

    #[test]
    fn test_endpoint_instrument() {
        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/instruments",
            Endpoint::Instrument(EndpointInstrument::Instruments).url_endpoint()
        );

        assert_eq!(
            "https://api.schwabapi.com/marketdata/v1/instruments/123456",
            Endpoint::Instrument(EndpointInstrument::Instrument { cusip_id: "123456" })
                .url_endpoint()
        );
    }
}
