//! A messenger that uses standard input/output.

use axum::extract::Query;
use http::Uri;
use oauth2::CsrfToken;

use crate::error::Error;
use crate::token::{auth::AuthContext, auth::AuthRequest, auth::ChannelMessenger};

#[derive(Debug)]
pub struct StdioMessenger {
    context: Option<AuthContext>,
}

impl StdioMessenger {
    pub fn new() -> Self {
        Self { context: None }
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

    async fn send_auth_message(&self) -> Result<(), Error> {
        let context = self.context.as_ref().expect("context not set");
        let auth_url = context.auth_url.as_ref().expect("auth url not set");
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

        println!("{}", message);
        Ok(())
    }

    async fn receive_auth_message(&self) -> Result<String, Error> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let uri: Uri = input.trim().parse().expect("right uri");

        let context = self.context.as_ref().expect("context not set");
        let csrf = context.csrf.as_ref().expect("auth csrf not set");

        Ok(Self::uri_to_auth_code(&uri, csrf))
    }
}
#[cfg(test)]
mod tests {
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
}
