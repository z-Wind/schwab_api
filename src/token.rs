//! Structs and utilities for Authorization.

pub(crate) mod auth;
pub(crate) mod local_server;
pub(crate) mod stdio_messenger;
pub(crate) mod utils;

use async_trait;
use chrono::TimeDelta;
use local_server::LocalServerMessenger;
use oauth2::TokenResponse;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use tokio::sync::Mutex;
use utils::ChannelMessenger;

use crate::error::Error;
use auth::Authorizer;

#[async_trait::async_trait]
pub trait Tokener {
    async fn get_access_token(&self) -> Result<String, Error>;

    async fn redo_authorization(&self) -> Result<(), Error>;
}

const ACCESS_TOKEN_LIFETIME: TimeDelta = TimeDelta::minutes(25); // 25 Minutes instead of 30 min
const REFRESH_TOKEN_LIFETIME: TimeDelta = TimeDelta::days(6); // 6 days instead of 7 days

#[derive(Debug)]
pub struct TokenChecker {
    path: PathBuf,
    authorizer: Mutex<Authorizer>,
    token: Mutex<Token>,
    messenger: Mutex<Box<dyn ChannelMessenger + Send>>,
}

impl TokenChecker {
    pub async fn new(
        path: PathBuf,
        client_id: String,
        secret: String,
        redirect_url: String,
        certs_dir: PathBuf,
        async_client: Client,
    ) -> Result<Self, Error> {
        let mut auth = Authorizer::new(
            client_id,
            secret,
            redirect_url,
            auth::AuthProcess::Auto { certs_dir },
            async_client,
        );
        let context = auth.create_auth_context();
        let mut messenger: Box<dyn ChannelMessenger + Send> =
            Box::new(LocalServerMessenger::new(&context).await);
        let token = match Token::load(path.clone()) {
            Ok(token) => token,
            Err(_) => auth.save(path.clone(), &mut messenger).await?,
        };

        let checker = Self {
            path,
            authorizer: Mutex::new(auth),
            token: Mutex::new(token),
            messenger: Mutex::new(messenger),
        };

        checker.check_or_update().await?;

        Ok(checker)
    }

    pub async fn new_with_custom_auth(
        path: PathBuf,
        client_id: String,
        secret: String,
        redirect_url: String,
        async_client: Client,
        mut messenger: Box<dyn ChannelMessenger + Send>,
    ) -> Result<Self, Error> {
        let mut auth = Authorizer::new(
            client_id,
            secret,
            redirect_url,
            auth::AuthProcess::Manual,
            async_client,
        );

        let token = match Token::load(path.clone()) {
            Ok(token) => token,
            Err(_) => auth.save(path.clone(), &mut messenger).await?,
        };

        let checker = Self {
            path,
            authorizer: Mutex::new(auth),
            token: Mutex::new(token),
            messenger: Mutex::new(messenger),
        };

        checker.check_or_update().await?;

        Ok(checker)
    }

    pub async fn new_with_auth_manually(
        path: PathBuf,
        client_id: String,
        secret: String,
        redirect_url: String,
        async_client: Client,
    ) -> Result<Self, Error> {
        let messenger: Box<dyn ChannelMessenger + Send> =
            Box::new(stdio_messenger::StdioMessenger::new());
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

    async fn check_or_update(&self) -> Result<(), Error> {
        let mut token = self.token.lock().await;
        if token.is_access_valid() {
            return Ok(());
        }

        if token.is_refresh_valid() {
            if let Ok(rsp) = self
                .authorizer
                .lock()
                .await
                .access_token(&token.refresh)
                .await
            {
                token.access.clone_from(rsp.access_token().secret());
                token.access_expires_in = chrono::Utc::now()
                    .checked_add_signed(ACCESS_TOKEN_LIFETIME)
                    .expect("access_expires_in");

                token.save(self.path.clone())?;

                return Ok(());
            }
        }

        let mut messenger = self.messenger.lock().await;
        *token = self
            .authorizer
            .lock()
            .await
            .save(self.path.clone(), &mut *messenger)
            .await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Tokener for TokenChecker {
    async fn get_access_token(&self) -> Result<String, Error> {
        self.check_or_update().await?;
        let access_token = self.token.lock().await.access.clone();
        Ok(access_token)
    }

    /// must update token in Tokener
    async fn redo_authorization(&self) -> Result<(), Error> {
        let mut authorizer = self.authorizer.lock().await;
        let mut messenger = self.messenger.lock().await;

        let fut = authorizer.save(self.path.clone(), &mut *messenger);
        let mut token = self.token.lock().await;
        *token = fut.await?;

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

    #[tokio::test]
    #[ignore = "Testing manually for browser verification. Should be --nocapture"]
    async fn test_token_checker_new() {
        let path = dirs::home_dir()
            .expect("home dir")
            .join(".credentials")
            .join("Schwab-rust.json");
        #[allow(clippy::option_env_unwrap)]
        let client_id = option_env!("SCHWAB_API_KEY").expect("There should be SCHWAB API KEY");
        #[allow(clippy::option_env_unwrap)]
        let secret = option_env!("SCHWAB_SECRET").expect("There should be SCHWAB SECRET");

        TokenChecker::new(
            path,
            client_id.to_string(),
            secret.to_string(),
            "https://127.0.0.1:8080".to_string(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs"),
            Client::new(),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    #[ignore = "Testing manually for browser verification. Should be --nocapture"]
    async fn test_token_checker_new_with_auth_manually() {
        let path = dirs::home_dir()
            .expect("home dir")
            .join(".credentials")
            .join("Schwab-rust.json");
        #[allow(clippy::option_env_unwrap)]
        let client_id = option_env!("SCHWAB_API_KEY").expect("There should be SCHWAB API KEY");
        #[allow(clippy::option_env_unwrap)]
        let secret = option_env!("SCHWAB_SECRET").expect("There should be SCHWAB SECRET");

        TokenChecker::new_with_auth_manually(
            path,
            client_id.to_string(),
            secret.to_string(),
            "https://127.0.0.1:8080".to_string(),
            Client::new(),
        )
        .await;
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
