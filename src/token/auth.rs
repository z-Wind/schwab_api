use oauth2::{
    basic::{BasicClient, BasicRequestTokenError, BasicTokenResponse},
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    HttpClientError, RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};
use reqwest::Client;
use serde::Deserialize;
use std::path::PathBuf;
use url::Url;

use super::channel_messenger::{AuthContext, ChannelMessenger};
use crate::error::Error;
use crate::token::Token;

type RequestTokenError = BasicRequestTokenError<HttpClientError<reqwest::Error>>;

#[derive(Debug, Deserialize)]
pub(super) struct AuthRequest {
    pub(super) code: String,
    pub(super) state: String,
}

#[derive(Debug)]
pub(super) struct Authorizer<CM: ChannelMessenger> {
    oauth2_client:
        BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,
    async_client: Client,
    messenger: CM,
}

impl<CM: ChannelMessenger> Authorizer<CM> {
    pub(super) async fn new(
        app_key: String,
        secret: String,
        redirect_url: String,
        async_client: Client,
        messenger: CM,
    ) -> Result<Self, Error> {
        let app_key = ClientId::new(app_key);
        let secret = ClientSecret::new(secret);
        let auth_url = AuthUrl::new("https://api.schwabapi.com/v1/oauth/authorize".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://api.schwabapi.com/v1/oauth/token".to_string())
            .expect("Invalid token endpoint URL");
        let redirect_url = RedirectUrl::new(redirect_url).expect("Invalid redirect URL");

        let oauth2_client = BasicClient::new(app_key)
            .set_client_secret(secret)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url);

        let mut auth = Authorizer {
            oauth2_client,
            async_client,
            messenger,
        };
        let context = auth.create_auth_context();
        auth.messenger.with_context(context).await?;

        Ok(auth)
    }

    async fn authorize(&self) -> Result<Token, Error> {
        let auth_code = {
            self.messenger.send_auth_message().await?;
            AuthorizationCode::new(
                self.messenger
                    .receive_auth_message()
                    .await
                    .expect("Failed to get auth message."),
            )
        };

        let token_result = self
            .refresh_token(auth_code)
            .await
            .map_err(|e| Error::Token(e.to_string()))?;

        // dbg!(&token_result);
        let token = Token {
            refresh: token_result
                .refresh_token()
                .expect("should have refresh_token")
                .secret()
                .to_string(),
            refresh_expires_in: chrono::Utc::now()
                .checked_add_signed(super::REFRESH_TOKEN_LIFETIME)
                .expect("refresh_expires_in"),
            access: token_result.access_token().secret().to_string(),
            access_expires_in: chrono::Utc::now()
                .checked_add_signed(super::ACCESS_TOKEN_LIFETIME)
                .expect("access_expires_in"),
            type_: token_result.token_type().as_ref().to_string(),
        };

        Ok(token)
    }

    fn auth_code_url(&self) -> (Url, CsrfToken) {
        let (auth_url, csrf_token) = self
            .oauth2_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("readonly".to_string()))
            .url();
        (auth_url, csrf_token)
    }

    async fn refresh_token(
        &self,
        auth_code: AuthorizationCode,
    ) -> Result<BasicTokenResponse, RequestTokenError> {
        self.oauth2_client
            .exchange_code(auth_code)
            .request_async(&self.async_client)
            .await
    }

    pub(super) async fn access_token(
        &self,
        refresh_token: &str,
    ) -> Result<BasicTokenResponse, RequestTokenError> {
        let refresh_token = RefreshToken::new(refresh_token.to_string());
        self.oauth2_client
            .exchange_refresh_token(&refresh_token)
            .request_async(&self.async_client)
            .await
    }

    fn create_auth_context(&self) -> AuthContext {
        let (auth_url, csrf_token) = self.auth_code_url();
        let context = AuthContext {
            auth_url: Some(auth_url),
            csrf: Some(csrf_token),
            redirect_url: Some(
                self.oauth2_client
                    .redirect_uri()
                    .expect("redirect_url")
                    .url()
                    .clone(),
            ),
        };
        context
    }

    pub(super) async fn save(&self, path: PathBuf) -> Result<Token, Error> {
        let token = self
            .authorize()
            .await
            .map_err(|e| Error::Token(e.to_string()))?;
        token.save(path)?;
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use std::{borrow::Cow, collections::HashMap};

    use crate::token::channel_messenger::compound_messenger::CompoundMessenger;
    use crate::token::channel_messenger::local_server::LocalServerMessenger;
    use crate::token::channel_messenger::stdio_messenger::StdioMessenger;

    fn client_id_static() -> &'static str {
        #[allow(clippy::option_env_unwrap)]
        option_env!("SCHWAB_API_KEY")
            .expect("The environment variable SCHWAB_API_KEY sholud be set")
    }

    fn secret_static() -> &'static str {
        #[allow(clippy::option_env_unwrap)]
        option_env!("SCHWAB_SECRET").expect("The environment variable SCHWAB_SECRET sholud be set")
    }

    fn callback_url_static() -> &'static str {
        #[allow(clippy::option_env_unwrap)]
        option_env!("SCHWAB_CALLBACK_URL")
            .expect("The environment variable SCHWAB_CALLBACK_URL sholud be set")
    }

    #[tokio::test]
    #[ignore = "Testing manually for compound verification. Should be --nocapture"]
    async fn test_auth_compound() {
        let certs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs");
        let messenger = CompoundMessenger::new(
            LocalServerMessenger::new(&certs_dir).await,
            StdioMessenger::new(),
        );

        let auth = Authorizer::new(
            client_id_static().to_string(),
            secret_static().to_string(),
            callback_url_static().to_string(),
            Client::new(),
            messenger,
        )
        .await
        .unwrap();

        let token = auth.authorize().await.unwrap();
        dbg!(&token);

        // test refresh access token
        let access_token = auth.access_token(&token.refresh).await.unwrap();
        dbg!(&access_token);
    }

    #[tokio::test]
    #[ignore = "Testing manually for browser verification. Should be --nocapture"]
    async fn test_auth_local_server() {
        let certs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs");
        let messenger = LocalServerMessenger::new(&certs_dir).await;

        let auth = Authorizer::new(
            client_id_static().to_string(),
            secret_static().to_string(),
            callback_url_static().to_string(),
            Client::new(),
            messenger,
        )
        .await
        .unwrap();

        let token = auth.authorize().await.unwrap();
        dbg!(&token);

        // test refresh access token
        let access_token = auth.access_token(&token.refresh).await.unwrap();
        dbg!(&access_token);
    }

    #[tokio::test]
    #[ignore = "Testing manually for stdio verification. Should be --nocapture"]
    async fn test_auth_stdio() {
        let messenger = StdioMessenger::new();

        let auth = Authorizer::new(
            client_id_static().to_string(),
            secret_static().to_string(),
            callback_url_static().to_string(),
            Client::new(),
            messenger,
        )
        .await
        .unwrap();

        let token = auth.authorize().await.unwrap();
        dbg!(&token);

        // test refresh access token
        let access_token = auth.access_token(&token.refresh).await.unwrap();
        dbg!(&access_token);
    }

    #[tokio::test]
    async fn test_get_auth_code_url() {
        const CLIENTID: &str = "CLIENTID";
        const SECRET: &str = "SECRET";
        const REDIRECT_URL: &str = "https://127.0.0.1:8080";
        let messenger = StdioMessenger::new();

        let auth = Authorizer::new(
            CLIENTID.to_string(),
            SECRET.to_string(),
            REDIRECT_URL.to_string(),
            Client::new(),
            messenger,
        )
        .await
        .unwrap();

        let (auth_url, csrf_token) = auth.auth_code_url();

        println!("{auth_url:?}");
        assert_eq!(auth_url.scheme(), "https");
        assert_eq!(auth_url.host_str().unwrap(), "api.schwabapi.com");
        assert_eq!(auth_url.path(), "/v1/oauth/authorize");
        let pairs: HashMap<_, _> = auth_url.query_pairs().into_iter().collect();
        assert_eq!(pairs.len(), 5);
        assert_eq!(
            pairs.get(&Cow::Borrowed("state")).unwrap(),
            &Cow::Borrowed(csrf_token.secret().as_str())
        );
        assert_eq!(
            pairs.get(&Cow::Borrowed("response_type")).unwrap(),
            &Cow::Borrowed("code")
        );
        assert_eq!(
            pairs.get(&Cow::Borrowed("client_id")).unwrap(),
            &Cow::Borrowed(CLIENTID)
        );
        assert_eq!(
            pairs.get(&Cow::Borrowed("redirect_uri")).unwrap(),
            &Cow::Borrowed(REDIRECT_URL)
        );
        assert_eq!(
            pairs.get(&Cow::Borrowed("scope")).unwrap(),
            &Cow::Borrowed("readonly")
        );
        assert!(!csrf_token.secret().is_empty());
    }
}
