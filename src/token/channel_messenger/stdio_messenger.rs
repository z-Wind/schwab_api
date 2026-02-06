//! A messenger that uses standard input/output.

use axum::extract::Query;
use http::Uri;
use oauth2::CsrfToken;
use tracing::instrument;

use super::{AuthContext, ChannelMessenger};
use crate::error::Error;
use crate::token::auth::AuthRequest;

#[derive(Debug, Default)]
pub struct StdioMessenger {
    context: Option<AuthContext>,
}

impl StdioMessenger {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn uri_to_auth_code(uri: &Uri, csrf: &CsrfToken) -> String {
        let Query(query): Query<AuthRequest> = Query::try_from_uri(uri).expect("right format");
        assert!(&query.state == csrf.secret(), "CSRF check error");

        query.code
    }
}

impl ChannelMessenger for StdioMessenger {
    async fn with_context(&mut self, context: AuthContext) -> Result<(), Error> {
        self.context = Some(context);
        Ok(())
    }

    #[instrument(skip(self))]
    async fn send_auth_message(&self) -> Result<(), Error> {
        let context = self.context.as_ref().ok_or_else(|| {
            let err = "missing context for authentication";
            tracing::error!(reason = %err);
            Error::ChannelMessenger(err.to_string())
        })?;

        let auth_url = context.auth_url.as_ref().ok_or_else(|| {
            let err = "missing auth_url in context";
            tracing::error!(reason = %err);
            Error::ChannelMessenger(err.to_string())
        })?;

        let message = format!(
            r#"
**************************************************************

This is the manual login and token creation flow for schwab_api.
Please follow these instructions exactly:

 1. Open the following link by copy-pasting it into the browser
    of your choice:

    {auth_url}

 2. Log in with your account credentials. You may be asked to
    perform two-factor authentication using text messaging or
    another method, as well as whether to trust the browser.

 3. When asked whether to allow your app access to your account,
    select "Allow".

 4. Your browser should be redirected to your callback URI. Copy
    the ENTIRE address, paste it into the following prompt, and press
    Enter/Return.

**************************************************************

Redirect URL>"#
        );

        println!("{message}");

        tracing::info!(
            url_len = %auth_url.as_str().len(),
            "authentication instructions displayed to user"
        );

        Ok(())
    }

    async fn receive_auth_message(&self) -> Result<String, Error> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let uri: Uri = input
            .trim()
            .parse()
            .map_err(|e| Error::ChannelMessenger(format!("{e:?}")))?;

        let context = self
            .context
            .as_ref()
            .ok_or(Error::ChannelMessenger("No context".to_string()))?;
        let csrf = context
            .csrf
            .as_ref()
            .ok_or(Error::ChannelMessenger("No CSRF".to_string()))?;

        Ok(Self::uri_to_auth_code(&uri, csrf))
    }
}
#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_uri_to_auth_code() {
        let csrf = CsrfToken::new("CSRF".to_string());
        let uri: Uri = format!("https://127.0.0.1:8080/?state={}&code=code", csrf.secret())
            .parse()
            .unwrap();
        let auth_code = StdioMessenger::uri_to_auth_code(&uri, &csrf);
        assert_eq!(auth_code, "code");
    }

    #[test(tokio::test)]
    #[ignore = "Testing manually for stdio verification. Should be --nocapture"]
    async fn test_stdio_messenger() {
        let context = AuthContext {
            auth_url: Some("https://127.0.0.1:8081".parse().unwrap()),
            csrf: Some(CsrfToken::new("CSRF".to_string())),
            redirect_url: Some("https://127.0.0.1:8081".parse().unwrap()),
        };

        let mut messenger = StdioMessenger::new();
        messenger.with_context(context).await.unwrap();
        messenger.send_auth_message().await.unwrap();

        // you should input https://127.0.0.1:8081/?state=CSRF&code=code
        assert_eq!("code", messenger.receive_auth_message().await.unwrap());
    }
}
