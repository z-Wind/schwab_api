use oauth2::{
    basic::{BasicClient, BasicRequestTokenError, BasicTokenResponse},
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    HttpClientError, RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};
use reqwest::Client;
use serde::Deserialize;
use std::path::PathBuf;
use url::Url;

use crate::error::Error;
use crate::token::utils::{AuthContext, ChannelMessenger};
use crate::token::Token;

type RequestTokenError = BasicRequestTokenError<HttpClientError<reqwest::Error>>;

#[derive(Debug)]
pub(super) enum AuthProcess {
    Auto { certs_dir: PathBuf },
    Manual,
}

#[derive(Debug, Deserialize)]
pub(super) struct AuthRequest {
    pub(super) code: String,
    pub(super) state: String,
}

#[derive(Debug)]
pub(super) struct Authorizer {
    oauth2_client:
        BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,
    process: AuthProcess,
    async_client: Client,
}

impl Authorizer {
    pub(super) fn new(
        app_key: String,
        secret: String,
        redirect_url: String,
        process: AuthProcess,
        async_client: Client,
    ) -> Self {
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

        Authorizer {
            oauth2_client,
            process,
            async_client,
        }
    }

    async fn authorize(
        &self,
        messenger: &mut Box<dyn ChannelMessenger + Send>,
    ) -> Result<Token, Box<dyn std::error::Error>> {
        let context = self.create_auth_context();
        messenger.with_context(context).await?;
        messenger.send_auth_message().await?;
        let auth_code = AuthorizationCode::new(
            messenger
                .receive_auth_message()
                .await
                .expect("Failed to get auth message."),
        );

        let token_result = self.refresh_token(auth_code).await?;
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

    pub fn create_auth_context(&self) -> AuthContext {
        let (auth_url, csrf_token) = self.auth_code_url();
        let context = AuthContext {
            url: Some(auth_url),
            csrf: Some(csrf_token),
            redirect_url: Some(
                self.oauth2_client
                    .redirect_uri()
                    .expect("redirect_url")
                    .url()
                    .clone(),
            ),
            certs_dir: match &self.process {
                AuthProcess::Auto { certs_dir } => Some(certs_dir.clone()),
                AuthProcess::Manual => None,
            },
        };
        context
    }

    pub(super) async fn save(
        &mut self,
        path: PathBuf,
        messenger: &mut Box<dyn ChannelMessenger + Send>,
    ) -> Result<Token, Error> {
        let token = self
            .authorize(messenger)
            .await
            .map_err(|e| Error::Token(e.to_string()))?;
        token.save(path)?;
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::token::local_server::LocalServerMessenger;
    use crate::token::stdio_messenger::StdioMessenger;

    use pretty_assertions::assert_eq;
    use std::{borrow::Cow, collections::HashMap};

    fn callback_url_static() -> &'static str {
        #[allow(clippy::option_env_unwrap)]
        option_env!("SCHWAB_CALLBACK_URL").expect("There should be SCHWAB CALLBACK URL")
    }

    fn client_id_static() -> &'static str {
        #[allow(clippy::option_env_unwrap)]
        option_env!("SCHWAB_API_KEY").expect("There should be SCHWAB API KEY")
    }

    fn secret_static() -> &'static str {
        #[allow(clippy::option_env_unwrap)]
        option_env!("SCHWAB_SECRET").expect("There should be SCHWAB SECRET")
    }

    #[tokio::test]
    #[ignore = "Testing manually for browser verification. Should be --nocapture"]
    async fn test_auth_auto() {
        let auth = Authorizer::new(
            client_id_static().to_string(),
            secret_static().to_string(),
            callback_url_static().to_string(),
            AuthProcess::Auto {
                certs_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs"),
            },
            Client::new(),
        );

        let context = auth.create_auth_context();
        let mut messenger: Box<dyn ChannelMessenger + Send> =
            Box::new(LocalServerMessenger::new(&context).await);
        let token = auth.authorize(&mut messenger).await.unwrap();
        dbg!(&token);

        // test refresh access token
        let access_token = auth.access_token(&token.refresh).await.unwrap();
        dbg!(&access_token);
    }

    #[tokio::test]
    #[ignore = "Testing manually for browser verification. Should be --nocapture"]
    async fn test_auth_manually() {
        let auth = Authorizer::new(
            client_id_static().to_string(),
            secret_static().to_string(),
            callback_url_static().to_string(),
            AuthProcess::Manual,
            Client::new(),
        );
        let mut messenger: Box<dyn ChannelMessenger + Send> = Box::new(StdioMessenger::new());
        let token = auth.authorize(&mut messenger).await.unwrap();
        dbg!(&token);

        // test refresh access token
        let access_token = auth.access_token(&token.refresh).await.unwrap();
        dbg!(&access_token);
    }

    #[test]
    fn test_get_auth_code_url() {
        const CLIENTID: &str = "CLIENTID";
        const SECRET: &str = "SECRET";
        const REDIRECT_URL: &str = "https://127.0.0.1:8080";
        let auth = Authorizer::new(
            CLIENTID.to_string(),
            SECRET.to_string(),
            REDIRECT_URL.to_string(),
            AuthProcess::Auto {
                certs_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs"),
            },
            Client::new(),
        );

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

    #[tokio::test]
    #[ignore = "If the test is performed manually on Linux, it may fail for HTTPS."]
    async fn test_get_auth_code_with_local_server() {
        async fn get_auth_code(redirect_url: Url) -> AuthorizationCode {
            let context = AuthContext {
                url: Some(redirect_url.clone()),
                csrf: Some(CsrfToken::new("CSRF".to_string())),
                redirect_url: Some(redirect_url.clone()),
                certs_dir: Some(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs")),
            };
            let messenger = LocalServerMessenger::new(&context).await;
            messenger.send_auth_message().await.unwrap();
            AuthorizationCode::new(messenger.receive_auth_message().await.unwrap())
        }

        let redirect_url = "https://127.0.0.1:8081".parse().unwrap();
        let auth_code = tokio::spawn(get_auth_code(redirect_url));
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        let body = client
            .get("https://127.0.0.1:8081/?state=CSRF&code=code")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        assert_eq!(auth_code.await.unwrap().secret(), "code");
        assert_eq!(body, "Schwab returned the following code:\ncode\nYou can now safely close this browser window.");
    }
}
