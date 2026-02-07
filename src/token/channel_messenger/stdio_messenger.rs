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

    #[instrument(skip(uri, csrf), fields(csrf_valid = tracing::field::Empty))]
    fn uri_to_auth_code(uri: &Uri, csrf: &CsrfToken) -> Result<String, Error> {
        tracing::debug!(path = %uri.path(), "parsing authorization callback");

        // 解析查詢參數
        let Query(query): Query<AuthRequest> = Query::try_from_uri(uri).map_err(|e| {
            tracing::error!(
                error = %e,
                "failed to parse URI query parameters; invalid callback format"
            );
            Error::Token(format!("Invalid callback URI format: {e}"))
        })?;

        tracing::debug!("successfully parsed callback query parameters");

        // CSRF 驗證
        if &query.state != csrf.secret() {
            tracing::Span::current().record("csrf_valid", false);
            tracing::error!("CSRF validation failed");
            return Err(Error::Token("CSRF token mismatch".to_string()));
        }

        tracing::Span::current().record("csrf_valid", true);
        tracing::debug!("CSRF token validated");

        tracing::debug!(
            code_length = query.code.len(),
            "extracted authorization code from callback"
        );

        Ok(query.code)
    }
}

impl ChannelMessenger for StdioMessenger {
    #[instrument(skip(self, context), fields(redirect_url = %context.redirect_url))]
    async fn with_context(&mut self, context: AuthContext) -> Result<(), Error> {
        tracing::debug!("configuring stdio messenger with auth context");
        self.context = Some(context);
        tracing::debug!("stdio messenger configured successfully");
        Ok(())
    }

    #[instrument(skip(self))]
    async fn send_auth_message(&self) -> Result<(), Error> {
        let context = self.context.as_ref().ok_or_else(|| {
            let err = "missing context for authentication";
            tracing::error!(reason = %err);
            Error::ChannelMessenger(err.to_string())
        })?;

        let auth_url = context.auth_url.clone();

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

    #[instrument(skip(self))]
    async fn receive_auth_message(&self) -> Result<String, Error> {
        tracing::info!("waiting for user to paste callback URL");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).inspect_err(|e| {
            tracing::error!(error = %e, "failed to read from stdin");
        })?;

        tracing::debug!(
            input_length = input.trim().len(),
            "received input from user"
        );

        let uri: Uri = input.trim().parse().map_err(|e| {
            tracing::error!(error = ?e, input = %input.trim(), "failed to parse input as URI");
            Error::ChannelMessenger(format!("Invalid URI format: {e:?}"))
        })?;

        let context = self.context.as_ref().ok_or_else(|| {
            tracing::error!("context not configured; with_context must be called first");
            Error::ChannelMessenger("No context configured".to_string())
        })?;

        let csrf = &context.csrf;

        Self::uri_to_auth_code(&uri, csrf)
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
        let auth_code = StdioMessenger::uri_to_auth_code(&uri, &csrf).unwrap();
        assert_eq!(auth_code, "code");
    }

    #[test]
    fn test_uri_to_auth_code_success() {
        let csrf = CsrfToken::new("test_state".to_string());
        let uri = Uri::from_static("/callback?code=test_code&state=test_state");

        let result = StdioMessenger::uri_to_auth_code(&uri, &csrf);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_code");
    }

    #[test]
    fn test_uri_to_auth_code_csrf_mismatch() {
        let csrf = CsrfToken::new("expected_state".to_string());
        let uri = Uri::from_static("/callback?code=test_code&state=wrong_state");

        let result = StdioMessenger::uri_to_auth_code(&uri, &csrf);
        assert!(matches!(result, Err(Error::Token(_))));
    }

    #[test]
    fn test_uri_to_auth_code_invalid_format() {
        let csrf = CsrfToken::new("test_state".to_string());
        let uri = Uri::from_static("/callback?invalid=params");

        let result = StdioMessenger::uri_to_auth_code(&uri, &csrf);
        assert!(matches!(result, Err(Error::Token(_))));
    }

    #[test(tokio::test)]
    #[ignore = "Testing manually for stdio verification. Should be --nocapture"]
    async fn test_stdio_messenger() {
        let context = AuthContext {
            auth_url: "https://127.0.0.1:8081".parse().unwrap(),
            csrf: CsrfToken::new("CSRF".to_string()),
            redirect_url: "https://127.0.0.1:8081".parse().unwrap(),
        };

        let mut messenger = StdioMessenger::new();
        messenger.with_context(context).await.unwrap();
        messenger.send_auth_message().await.unwrap();

        // you should input https://127.0.0.1:8081/?state=CSRF&code=code
        assert_eq!("code", messenger.receive_auth_message().await.unwrap());
    }
}
