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
}
