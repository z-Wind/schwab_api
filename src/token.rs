pub(crate) mod auth;
pub(crate) mod local_server;

use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::sync::Mutex;

use crate::error::Error;
use auth::Authorizer;

const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;
const DAY: u64 = 24 * HOUR;
const ACCESS_TOKEN_LIFETIME: u64 = 25 * MINUTE; // 25 Minutes instead of 30 min
const REFRESH_TOKEN_LIFETIME: u64 = 60 * DAY; // 60 days instead of 90 days

#[derive(Debug)]
pub(crate) struct TokenChecker {
    path: PathBuf,
    authorizer: Authorizer,
    token: Mutex<Token>,
}

impl TokenChecker {
    pub(crate) async fn new(
        path: PathBuf,
        client_id: String,
        secret: String,
        redirect_url: String,
        certs_dir: PathBuf,
    ) -> Result<Self, Error> {
        let auth = Authorizer::new(client_id, secret, redirect_url, certs_dir);
        let token = match Token::load(path.clone()) {
            Ok(token) => token,
            Err(_) => auth.save(path.clone()).await?,
        };

        let checker = Self {
            path,
            authorizer: auth,
            token: Mutex::new(token),
        };

        checker.check_or_update().await?;

        Ok(checker)
    }

    pub(crate) async fn get_access_token(&self) -> Result<String, Error> {
        self.check_or_update().await?;
        let access_token = self.token.lock().await.access.clone();
        Ok(access_token)
    }

    async fn check_or_update(&self) -> Result<(), Error> {
        let mut token = self.token.lock().await;
        if token.is_access_valid() {
            return Ok(());
        }

        if token.is_refresh_valid() {
            let rsp = self
                .authorizer
                .access_token(&token.refresh)
                .await
                .map_err(|e| Error::TokenError(e.to_string()))?;
            token.access = rsp.access_token().secret().clone();
            token.access_expires_in = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + ACCESS_TOKEN_LIFETIME;

            token.save(self.path.clone())?;

            return Ok(());
        }

        *token = self.authorizer.save(self.path.clone()).await?;

        Ok(())
    }
}

// Define a struct to hold the OAuth2 token
#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Token {
    refresh: String,
    refresh_expires_in: u64,
    access: String,
    access_expires_in: u64,
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
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            < self.refresh_expires_in
    }

    fn is_access_valid(&self) -> bool {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            < self.access_expires_in
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
            refresh_expires_in: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - 1,
            access_expires_in: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - 1,
            ..Default::default()
        };

        assert!(!token.is_refresh_valid());
        assert!(!token.is_access_valid());

        let token = Token {
            refresh_expires_in: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 1,
            access_expires_in: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 1,
            ..Default::default()
        };

        assert!(token.is_refresh_valid());
        assert!(token.is_access_valid());
    }
}
