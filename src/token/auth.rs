use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    HttpClientError, RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
    basic::{BasicClient, BasicRequestTokenError, BasicTokenResponse},
};
use reqwest::Client;
use serde::Deserialize;
use std::{path::PathBuf, sync::LazyLock};
use tracing::instrument;
use url::Url;

use super::channel_messenger::{AuthContext, ChannelMessenger};
use crate::error::Error;
use crate::token::Token;

const SCHWAB_AUTH_URL: &str = "https://api.schwabapi.com/v1/oauth/authorize";
const SCHWAB_TOKEN_URL: &str = "https://api.schwabapi.com/v1/oauth/token";

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
    redirect_url: Url,
}

impl<CM: ChannelMessenger> Authorizer<CM> {
    #[instrument(skip_all, fields(redirect_url = %redirect_url))]
    pub(super) async fn new(
        app_key: String,
        secret: String,
        redirect_url: String,
        async_client: Client,
        messenger: CM,
    ) -> Result<Self, Error> {
        static AUTH_URL: LazyLock<AuthUrl> = LazyLock::new(|| {
            AuthUrl::new(SCHWAB_AUTH_URL.to_string()).expect("Invalid SCHWAB_AUTH_URL")
        });
        static TOKEN_URL: LazyLock<TokenUrl> = LazyLock::new(|| {
            TokenUrl::new(SCHWAB_TOKEN_URL.to_string()).expect("Invalid SCHWAB_TOKEN_URL")
        });

        let redirect_url = RedirectUrl::new(redirect_url).map_err(|e| {
            tracing::error!(error = %e, "invalid redirect URL provided");
            Error::Config(format!("Invalid redirect URL: {e}"))
        })?;

        let redirect_url_raw = redirect_url.url().clone();

        let oauth2_client = BasicClient::new(ClientId::new(app_key))
            .set_client_secret(ClientSecret::new(secret))
            .set_auth_uri(AUTH_URL.clone())
            .set_token_uri(TOKEN_URL.clone())
            .set_redirect_uri(redirect_url);

        let mut auth = Authorizer {
            oauth2_client,
            async_client,
            messenger,
            redirect_url: redirect_url_raw,
        };

        tracing::debug!("creating authorization context");
        let context = auth.create_auth_context();

        auth.messenger
            .with_context(context)
            .await
            .inspect_err(|e| {
                tracing::error!(error = %e, "failed to initialize messenger with auth context");
            })?;

        tracing::info!("Schwab OAuth2 authorizer initialized successfully");
        Ok(auth)
    }

    #[instrument(skip(self))]
    async fn authorize(&self) -> Result<Token, Error> {
        let auth_code = {
            self.messenger.send_auth_message().await?;

            tracing::debug!("waiting for user to provide authorization code/URL");
            let raw_message = self
                .messenger
                .receive_auth_message()
                .await
                .inspect_err(|e| {
                    tracing::error!(error = %e, "failed to receive authorization message");
                })?;

            AuthorizationCode::new(raw_message)
        };

        tracing::info!("exchanging authorization code for tokens");
        let token_result = self.refresh_token(auth_code).await.map_err(|e| {
            tracing::error!(error = %e, "token exchange failed");
            Error::Token(e.to_string())
        })?;

        let token = Token {
            refresh: token_result
                .refresh_token()
                .ok_or_else(|| {
                    tracing::error!("missing refresh_token in API response");
                    Error::Token("No refresh token received".into())
                })?
                .secret()
                .clone(),
            refresh_expires_in: chrono::Utc::now()
                .checked_add_signed(super::REFRESH_TOKEN_LIFETIME)
                .ok_or_else(|| Error::Token("Invalid refresh expiration calculation".into()))?,

            access: token_result.access_token().secret().clone(),
            access_expires_in: chrono::Utc::now()
                .checked_add_signed(super::ACCESS_TOKEN_LIFETIME)
                .ok_or_else(|| Error::Token("Invalid access expiration calculation".into()))?,

            type_: token_result.token_type().as_ref().to_string(),
        };

        tracing::info!(
            token_type = %token.type_,
            "token authorization completed successfully"
        );

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

    #[instrument(skip(self, auth_code))]
    async fn refresh_token(
        &self,
        auth_code: AuthorizationCode,
    ) -> Result<BasicTokenResponse, RequestTokenError> {
        self.oauth2_client
            .exchange_code(auth_code)
            .request_async(&self.async_client)
            .await
    }

    #[instrument(skip(self, refresh_token))]
    pub(super) async fn access_token(
        &self,
        refresh_token: &str,
    ) -> Result<BasicTokenResponse, RequestTokenError> {
        tracing::debug!("exchanging refresh token for new access token");

        let refresh_token = RefreshToken::new(refresh_token.to_string());
        let result = self
            .oauth2_client
            .exchange_refresh_token(&refresh_token)
            .request_async(&self.async_client)
            .await?;

        tracing::info!("access token refreshed successfully via refresh token");
        Ok(result)
    }

    fn create_auth_context(&self) -> AuthContext {
        let (auth_url, csrf_token) = self.auth_code_url();

        AuthContext {
            auth_url,
            csrf: csrf_token,
            redirect_url: self.redirect_url.clone(),
        }
    }

    #[instrument(skip(self), fields(path = %path.display()))]
    pub(super) async fn save(&self, path: &PathBuf) -> Result<Token, Error> {
        tracing::info!("starting full authorization and token save flow");

        let token = self
            .authorize()
            .await
            .map_err(|e| Error::Token(e.to_string()))?;

        token.save(path)?;

        tracing::info!("token authorized and saved successfully");
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::{borrow::Cow, collections::HashMap};
    use test_log::test;

    use crate::token::channel_messenger::compound_messenger::CompoundMessenger;
    use crate::token::channel_messenger::local_server::LocalServerMessenger;
    use crate::token::channel_messenger::stdio_messenger::StdioMessenger;

    use super::*;

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

    #[test(tokio::test)]
    #[ignore = "Testing manually for compound verification. Should be --nocapture"]
    async fn test_auth_compound() {
        let certs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs");
        let messenger = CompoundMessenger::new(
            LocalServerMessenger::new(&certs_dir).await.unwrap(),
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
        tracing::debug!(?token);

        // test refresh access token
        let access_token = auth.access_token(&token.refresh).await.unwrap();
        tracing::debug!(?access_token);
    }

    #[test(tokio::test)]
    #[ignore = "Testing manually for browser verification. Should be --nocapture"]
    async fn test_auth_local_server() {
        let certs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs");
        let messenger = LocalServerMessenger::new(&certs_dir).await.unwrap();

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
        tracing::debug!(?token);

        // test refresh access token
        let access_token = auth.access_token(&token.refresh).await.unwrap();
        tracing::debug!(?access_token);
    }

    #[test(tokio::test)]
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
        tracing::debug!(?token);

        // test refresh access token
        let access_token = auth.access_token(&token.refresh).await.unwrap();
        tracing::debug!(?access_token);
    }

    #[test(tokio::test)]
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

        tracing::debug!(?auth_url);
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
