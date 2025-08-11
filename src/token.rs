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
    pub async fn new_with_custom_auth(
        path: PathBuf,
        client_id: String,
        secret: String,
        redirect_url: String,
        async_client: Client,
        messenger: CM,
    ) -> Result<Self, Error> {
        let authorizer =
            Authorizer::new(client_id, secret, redirect_url, async_client, messenger).await?;

        let token = match Token::load(path.clone()) {
            Ok(token) => token,
            Err(_) => authorizer.save(path.clone()).await?,
        };

        let checker = Self {
            path,
            authorizer,
            token: Mutex::new(token),
        };

        checker.check_or_update().await?;

        Ok(checker)
    }

    async fn check_or_update(&self) -> Result<(), Error> {
        let mut token = self.token.lock().await;
        if token.is_access_valid() {
            return Ok(());
        }

        if token.is_refresh_valid()
            && let Ok(rsp) = self.authorizer.access_token(&token.refresh).await
        {
            token.access.clone_from(rsp.access_token().secret());
            token.access_expires_in = chrono::Utc::now()
                .checked_add_signed(ACCESS_TOKEN_LIFETIME)
                .expect("access_expires_in");

            token.save(self.path.clone())?;

            return Ok(());
        }

        *token = self.authorizer.save(self.path.clone()).await?;
        Ok(())
    }
}

impl TokenChecker<LocalServerMessenger> {
    pub async fn new_with_local_server(
        path: PathBuf,
        client_id: String,
        secret: String,
        redirect_url: String,
        certs_dir: PathBuf,
        async_client: Client,
    ) -> Result<Self, Error> {
        let messenger = LocalServerMessenger::new(&certs_dir).await;

        let authorizer =
            Authorizer::new(client_id, secret, redirect_url, async_client, messenger).await?;

        let token = match Token::load(path.clone()) {
            Ok(token) => token,
            Err(_) => authorizer.save(path.clone()).await?,
        };

        let checker = Self {
            path,
            authorizer,
            token: Mutex::new(token),
        };

        checker.check_or_update().await?;

        Ok(checker)
    }
}

impl TokenChecker<StdioMessenger> {
    pub async fn new_with_stdio(
        path: PathBuf,
        client_id: String,
        secret: String,
        redirect_url: String,
        async_client: Client,
    ) -> Result<Self, Error> {
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
    async fn get_access_token(&self) -> Result<String, Error> {
        self.check_or_update().await?;
        let access_token = self.token.lock().await.access.clone();
        Ok(access_token)
    }

    /// must update token in Tokener
    async fn redo_authorization(&self) -> Result<(), Error> {
        let mut token = self.token.lock().await;
        *token = self.authorizer.save(self.path.clone()).await?;

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
    fn load(path: PathBuf) -> std::io::Result<Token> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let token: Token = serde_json::from_str(&contents)?;
        Ok(token)
    }

    fn save(&self, path: PathBuf) -> std::io::Result<()> {
        // println!("...save token: {}", path.display());
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        let json = serde_json::to_string_pretty(self)?;
        file.write_all(json.as_bytes())?;
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
    use super::*;

    use channel_messenger::compound_messenger::CompoundMessenger;

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
    #[ignore = "Testing manually for verification. Should be --nocapture"]
    async fn test_token_checker_new_with_custom_auth() {
        let path = dirs::home_dir()
            .expect("home dir")
            .join(".credentials")
            .join("Schwab-rust.json");

        let certs_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/certs"));
        let messenger = CompoundMessenger::new(
            LocalServerMessenger::new(&certs_dir).await,
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

    #[tokio::test]
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

    #[tokio::test]
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

        Token::save(&Token::default(), path).unwrap();
    }

    #[test]
    fn test_load_token() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("schwab")
            .join("token")
            .join("normal.json");

        let token = Token::load(path).unwrap();
        println!("{token:?}");
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
