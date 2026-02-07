//! Structs and utilities for Authorization.

pub(crate) mod auth;
pub mod channel_messenger;

use chrono::TimeDelta;
use oauth2::TokenResponse;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use tokio::sync::Mutex;
use tracing::instrument;

use crate::error::Error;
use auth::Authorizer;
use channel_messenger::ChannelMessenger;
use channel_messenger::local_server::LocalServerMessenger;
use channel_messenger::stdio_messenger::StdioMessenger;

pub trait Tokener {
    fn get_access_token(&self) -> impl std::future::Future<Output = Result<String, Error>> + Send;

    fn redo_authorization(&self) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

const ACCESS_TOKEN_LIFETIME: TimeDelta = TimeDelta::minutes(25); // 25 Minutes instead of 30 min
const REFRESH_TOKEN_LIFETIME: TimeDelta = TimeDelta::days(6); // 6 days instead of 7 days

#[derive(Debug)]
pub struct TokenChecker<CM: ChannelMessenger> {
    path: PathBuf,
    authorizer: Authorizer<CM>,
    token: Mutex<Token>,
}

impl<CM: ChannelMessenger> TokenChecker<CM> {
    #[instrument(
        skip(client_id, secret, async_client, messenger),
        fields(path = %path.display(), redirect_url = %redirect_url)
    )]
    pub async fn new_with_custom_auth(
        path: PathBuf,
        client_id: String,
        secret: String,
        redirect_url: String,
        async_client: Client,
        messenger: CM,
    ) -> Result<Self, Error> {
        tracing::info!("initializing token checker with custom auth messenger");

        let authorizer =
            Authorizer::new(client_id, secret, redirect_url, async_client, messenger).await?;

        tracing::debug!("attempting to load existing token");
        let token = match Token::load(&path) {
            Ok(token) => {
                tracing::info!("existing token loaded successfully");
                token
            }
            Err(e) => {
                tracing::warn!(error = %e, "no existing token found; starting full authorization");
                authorizer.save(&path).await?
            }
        };

        let checker = Self {
            path,
            authorizer,
            token: Mutex::new(token),
        };

        tracing::debug!("performing initial token check/update");
        checker.check_or_update().await?;

        tracing::info!("token checker initialized successfully");
        Ok(checker)
    }

    #[instrument(skip(self))]
    async fn check_or_update(&self) -> Result<(), Error> {
        let mut token = self.token.lock().await;

        if token.is_access_valid() {
            tracing::trace!("access token is still valid; skipping update");
            return Ok(());
        }

        tracing::info!("access token expired; attempting to refresh");

        if token.is_refresh_valid() {
            match self.authorizer.access_token(&token.refresh).await {
                Ok(rsp) => {
                    token.access.clone_from(rsp.access_token().secret());
                    token.access_expires_in = chrono::Utc::now()
                        .checked_add_signed(ACCESS_TOKEN_LIFETIME)
                        .ok_or_else(|| {
                            tracing::error!("failed to calculate access token expiration");
                            Error::Token("Expiration calculation overflow".into())
                        })?;

                    token.save(&self.path).map_err(|e| {
                        tracing::error!(error = %e, "failed to save updated token to disk");
                        e
                    })?;

                    tracing::info!("access token refreshed successfully via refresh_token");
                    return Ok(());
                }
                Err(e) => {
                    tracing::warn!(
                        error = %e,
                        "failed to refresh access token; falling back to full re-authorization"
                    );
                }
            }
        } else {
            tracing::warn!("refresh token is expired or missing; full re-authorization required");
        }

        tracing::info!("starting full authorization flow");
        let new_token = self.authorizer.save(&self.path).await.map_err(|e| {
            tracing::error!(error = %e, "full authorization flow failed");
            e
        })?;

        *token = new_token;

        tracing::info!("token updated and saved via full re-authorization");
        Ok(())
    }
}

impl TokenChecker<LocalServerMessenger> {
    #[instrument(
        skip(client_id, secret, async_client),
        fields(
            path = %path.display(),
            redirect_url = %redirect_url,
            certs_dir = %certs_dir.display()
        ),
    )]
    pub async fn new_with_local_server(
        path: PathBuf,
        client_id: String,
        secret: String,
        redirect_url: String,
        certs_dir: PathBuf,
        async_client: Client,
    ) -> Result<Self, Error> {
        tracing::info!("initializing token checker with local server messenger");

        let messenger = LocalServerMessenger::new(&certs_dir).await?;

        let authorizer =
            Authorizer::new(client_id, secret, redirect_url, async_client, messenger).await?;

        tracing::debug!("attempting to load existing token");
        let token = match Token::load(&path) {
            Ok(token) => {
                tracing::info!("existing token loaded successfully");
                token
            }
            Err(e) => {
                tracing::warn!(error = %e, "no existing token found; starting full authorization");
                authorizer.save(&path).await?
            }
        };

        let checker = Self {
            path,
            authorizer,
            token: Mutex::new(token),
        };

        tracing::debug!("performing initial token check/update");
        checker.check_or_update().await?;

        tracing::info!("token checker with local server initialized successfully");
        Ok(checker)
    }
}

impl TokenChecker<StdioMessenger> {
    #[instrument(
        skip(client_id, secret, async_client),
        fields(path = %path.display(), redirect_url = %redirect_url)
    )]
    pub async fn new_with_stdio(
        path: PathBuf,
        client_id: String,
        secret: String,
        redirect_url: String,
        async_client: Client,
    ) -> Result<Self, Error> {
        tracing::info!("initializing token checker with stdio messenger");

        let messenger = StdioMessenger::new();
        Self::new_with_custom_auth(
            path,
            client_id,
            secret,
            redirect_url,
            async_client,
            messenger,
        )
        .await
    }
}

impl<CM: ChannelMessenger> Tokener for TokenChecker<CM> {
    #[instrument(skip(self))]
    async fn get_access_token(&self) -> Result<String, Error> {
        tracing::trace!("retrieving access token");
        self.check_or_update().await?;
        let access_token = self.token.lock().await.access.clone();
        tracing::trace!(token_length = access_token.len(), "access token retrieved");
        Ok(access_token)
    }

    /// must update token in Tokener
    #[instrument(skip(self))]
    async fn redo_authorization(&self) -> Result<(), Error> {
        tracing::warn!("forcing full re-authorization (manual override)");
        let mut token = self.token.lock().await;
        *token = self.authorizer.save(&self.path).await?;
        tracing::info!("full re-authorization completed successfully");
        Ok(())
    }
}

// Define a struct to hold the OAuth2 token
#[derive(Serialize, Deserialize, Debug, Default)]
struct Token {
    refresh: String,
    refresh_expires_in: chrono::DateTime<chrono::Utc>,
    access: String,
    access_expires_in: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "type")]
    type_: String,
}

impl Token {
    #[instrument(fields(path = %path.display()))]
    fn load(path: &PathBuf) -> std::io::Result<Token> {
        tracing::debug!("loading token from file");

        let mut file = File::open(path).map_err(|e| {
            tracing::error!(error = %e, "failed to open token file");
            e
        })?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| {
            tracing::error!(error = %e, "failed to read token file contents");
            e
        })?;

        let token: Token = serde_json::from_str(&contents).map_err(|e| {
            tracing::error!(error = %e, "failed to deserialize token JSON");
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })?;

        tracing::info!("token loaded successfully from file");
        Ok(token)
    }

    #[instrument(skip(self), fields(path = %path.display()))]
    fn save(&self, path: &PathBuf) -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| {
                tracing::error!(directory = %dir.display(), error = %e, "Failed to create token directory");
                e
            })?;
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to open token file for writing");
                e
            })?;

        let json = serde_json::to_string_pretty(self).map_err(|e| {
            tracing::error!(error = %e, "Failed to serialize token to JSON");
            e
        })?;

        file.write_all(json.as_bytes()).map_err(|e| {
            tracing::error!(error = %e, "Failed to write bytes to token file");
            e
        })?;

        tracing::trace!("Token saved successfully");

        Ok(())
    }

    fn is_refresh_valid(&self) -> bool {
        chrono::Utc::now() < self.refresh_expires_in
    }

    fn is_access_valid(&self) -> bool {
        chrono::Utc::now() < self.access_expires_in
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use channel_messenger::compound_messenger::CompoundMessenger;

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
    #[ignore = "Testing manually for verification. Should be --nocapture"]
    async fn test_token_checker_new_with_custom_auth() {
        let path = dirs::home_dir()
            .expect("home dir")
            .join(".credentials")
            .join("Schwab-rust.json");

        let certs_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/certs"));
        let messenger = CompoundMessenger::new(
            LocalServerMessenger::new(&certs_dir).await.unwrap(),
            StdioMessenger::new(),
        );

        TokenChecker::new_with_custom_auth(
            path,
            client_id_static().to_string(),
            secret_static().to_string(),
            callback_url_static().to_string(),
            Client::new(),
            messenger,
        )
        .await
        .unwrap();
    }

    #[test(tokio::test)]
    #[ignore = "Testing manually for browser verification. Should be --nocapture"]
    async fn test_token_checker_new_with_local_server() {
        let path = dirs::home_dir()
            .expect("home dir")
            .join(".credentials")
            .join("Schwab-rust.json");

        TokenChecker::new_with_local_server(
            path,
            client_id_static().to_string(),
            secret_static().to_string(),
            callback_url_static().to_string(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs"),
            Client::new(),
        )
        .await
        .unwrap();
    }

    #[test(tokio::test)]
    #[ignore = "Testing manually for stdio verification. Should be --nocapture"]
    async fn test_token_checker_new_with_stdio() {
        let path = dirs::home_dir()
            .expect("home dir")
            .join(".credentials")
            .join("Schwab-rust.json");

        TokenChecker::new_with_stdio(
            path,
            client_id_static().to_string(),
            secret_static().to_string(),
            callback_url_static().to_string(),
            Client::new(),
        )
        .await
        .unwrap();
    }

    #[test]
    fn test_save_token() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("schwab")
            .join("token")
            .join("normal.json");

        Token::save(&Token::default(), &path).unwrap();
    }

    #[test]
    fn test_load_token() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("schwab")
            .join("token")
            .join("normal.json");

        let token = Token::load(&path).unwrap();

        tracing::debug!(?token, "Token 載入成功");
    }

    #[test]
    fn test_token_expire_in() {
        let token = Token {
            refresh_expires_in: chrono::Utc::now()
                .checked_sub_days(chrono::Days::new(1))
                .unwrap(),
            access_expires_in: chrono::Utc::now()
                .checked_sub_days(chrono::Days::new(1))
                .unwrap(),
            ..Default::default()
        };

        assert!(!token.is_refresh_valid());
        assert!(!token.is_access_valid());

        let token = Token {
            refresh_expires_in: chrono::Utc::now()
                .checked_add_days(chrono::Days::new(1))
                .unwrap(),
            access_expires_in: chrono::Utc::now()
                .checked_add_days(chrono::Days::new(1))
                .unwrap(),
            ..Default::default()
        };

        assert!(token.is_refresh_valid());
        assert!(token.is_access_valid());
    }
}
