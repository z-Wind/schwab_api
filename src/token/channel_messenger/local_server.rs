use async_channel::Receiver;
use axum::{
    Router,
    extract::{FromRef, Query, State},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use axum_server::tls_rustls::RustlsConfig;
use http::StatusCode;
use oauth2::CsrfToken;
use std::{net::SocketAddr, path::Path, result::Result};
use tokio::time::{Duration, timeout};
use tracing::instrument;
use url::Url;

use super::{AuthContext, ChannelMessenger};
use crate::{error::Error, token::auth::AuthRequest};

#[derive(Debug)]
pub struct LocalServerMessenger {
    config: RustlsConfig,

    addr: Option<SocketAddr>,
    rx: Option<Receiver<String>>,
    app_state: Option<AppState>,
    auth_url: Option<Url>,
}

impl LocalServerMessenger {
    /// # Errors
    ///
    /// Returns an error if:
    /// - `cert.pem` or `key.pem` files are missing
    /// - Certificate files are invalid or corrupted
    /// - Files cannot be read due to permissions
    #[tracing::instrument(fields(certs_dir = %certs_dir.display()))]
    pub async fn new(certs_dir: &Path) -> Result<Self, Error> {
        tracing::info!("initializing HTTPS server configuration");

        let cert_path = certs_dir.join("cert.pem");
        let key_path = certs_dir.join("key.pem");

        if !cert_path.exists() {
            tracing::error!(path = %cert_path.display(), "certificate file not found");
            return Err(Error::Config(format!(
                "Certificate file not found: {}",
                cert_path.display()
            )));
        }

        if !key_path.exists() {
            tracing::error!(path = %key_path.display(), "private key file not found");
            return Err(Error::Config(format!(
                "Private key file not found: {}",
                key_path.display()
            )));
        }

        tracing::debug!(
            cert_path = %cert_path.display(),
            key_path = %key_path.display(),
            "loading TLS certificates"
        );

        let config = RustlsConfig::from_pem_file(cert_path, key_path)
        .await
        .map_err(|e| {
            tracing::error!(
                error = %e,
                certs_dir = %certs_dir.display(),
                "failed to load TLS certificates; ensure cert.pem and key.pem exist and are valid"
            );
            Error::Config(format!("Failed to load certificates: {}", e))
        })?;

        tracing::info!("HTTPS server configuration initialized successfully");

        Ok(Self {
            config,
            addr: None,
            rx: None,
            app_state: None,
            auth_url: None,
        })
    }
}

impl ChannelMessenger for LocalServerMessenger {
    async fn with_context(&mut self, context: AuthContext) -> Result<(), Error> {
        let (tx, rx) = async_channel::unbounded();
        let csrf = context.csrf;
        let redirect_uri = context.redirect_url;

        self.app_state = Some(AppState { csrf, tx });
        self.rx = Some(rx);
        self.auth_url = Some(context.auth_url);
        self.addr = Some(parse_socket_addr(&redirect_uri).map_err(Error::ChannelMessenger)?);

        Ok(())
    }

    #[instrument(skip(self))]
    async fn send_auth_message(&self) -> Result<(), Error> {
        open::that(
            self.auth_url
                .as_ref()
                .ok_or(Error::ChannelMessenger("No auth_url".to_string()))?
                .as_ref(),
        )?;

        Ok(())
    }

    async fn receive_auth_message(&self) -> Result<String, Error> {
        let service = app(self
            .app_state
            .as_ref()
            .ok_or(Error::ChannelMessenger("No app_state".to_string()))?
            .clone())
        .into_make_service();
        tokio::spawn(
            axum_server::bind_rustls(
                *self
                    .addr
                    .as_ref()
                    .ok_or(Error::ChannelMessenger("No SocketAddr".to_string()))?,
                self.config.clone(),
            )
            .serve(service),
        );

        let code = self
            .rx
            .as_ref()
            .ok_or(Error::ChannelMessenger("No rx".to_string()))?
            .recv()
            .await
            .map_err(|e| Error::ChannelMessenger(format!("{e:?}")))?;
        Ok(code)
    }
}

fn app(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_code))
        .with_state(app_state)
}

#[derive(Debug, Clone, FromRef)]
struct AppState {
    csrf: CsrfToken,
    tx: async_channel::Sender<String>,
}

#[tracing::instrument(
    skip(csrf, tx),
    fields(
        csrf_valid = tracing::field::Empty,  // 稍後填入
    ),
)]
async fn get_code(
    Query(query): Query<AuthRequest>,
    State(csrf): State<CsrfToken>,
    State(tx): State<async_channel::Sender<String>>,
) -> Response {
    tracing::debug!("received OAuth callback request");

    if &query.state != csrf.secret() {
        tracing::warn!(
            received_state = %query.state,
            "CSRF token validation failed; potential security attack or stale request"
        );
        tracing::Span::current().record("csrf_valid", false);
        return (
            StatusCode::FORBIDDEN,
            Html("<h1>Authentication Failed</h1><p>Security validation failed. Please restart the authentication process.</p>")
        ).into_response();
    }

    tracing::Span::current().record("csrf_valid", true);
    tracing::info!("CSRF token validated successfully");

    tracing::debug!(
        code_length = query.code.len(),
        "received authorization code"
    );

    match timeout(Duration::from_secs(5), tx.send(query.code.clone())).await {
        Ok(Ok(())) => {
            tracing::info!("authorization code sent successfully");
        }
        Ok(Err(e)) => {
            tracing::error!(
                error = %e,
                "failed to send authorization code through channel; receiver may have been dropped"
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("<h1>Error</h1><p>Failed to process authorization. Please try again.</p>"),
            )
                .into_response();
        }
        Err(_) => {
            tracing::error!("timeout sending authorization code to channel");
            return (
                StatusCode::GATEWAY_TIMEOUT,
                Html("<h1>Error</h1><p>Timeout processing authorization. Please try again.</p>"),
            )
                .into_response();
        }
    }

    tracing::info!("authorization code sent successfully");

    Html(
        r#"
    <html>
    <head><title>Authorization Successful</title></head>
    <body>
        <h1>✓ Authorization Successful</h1>
        <p>You can now safely close this window.</p>
    </body>
    </html>
    "#,
    )
    .into_response()
}

fn parse_socket_addr(url: &Url) -> Result<SocketAddr, String> {
    let Some(hostname) = url.host_str() else {
        return Err("No hostname found in URL".to_string());
    };

    let port = url.port().unwrap_or(443); // default to HTTPS port if not specified

    let addr = format!("{hostname}:{port}");
    match addr.parse::<SocketAddr>() {
        Ok(addr) => Ok(addr),
        Err(err) => Err(format!("Failed to parse socket address: {err}")),
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode, Uri},
    };
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;
    use test_log::test;
    use tower::ServiceExt; // for `oneshot` and `ready`

    use super::*;

    fn config(csrf: CsrfToken, tx: async_channel::Sender<String>) -> AppState {
        AppState { csrf, tx }
    }

    #[test]
    fn test_parse_socket_addr() {
        // Valid URL with specified port
        let expected_addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let addr = parse_socket_addr(&"https://127.0.0.1:8080".parse().unwrap()).unwrap();
        assert_eq!(addr, expected_addr);

        // Valid URL with default HTTPS port
        let expected_addr = SocketAddr::from(([127, 0, 0, 1], 443));
        let addr = parse_socket_addr(&"https://127.0.0.1".parse().unwrap()).unwrap();
        assert_eq!(addr, expected_addr);

        // URL without hostname
        let err = parse_socket_addr(&"https:///path".parse().unwrap()).unwrap_err();
        assert_eq!(
            err,
            "Failed to parse socket address: invalid socket address syntax"
        );

        // URL with non-standard port
        let expected_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let addr = parse_socket_addr(&"https://127.0.0.1:3000".parse().unwrap()).unwrap();
        assert_eq!(addr, expected_addr);

        // URL with IP address and port
        let expected_addr = SocketAddr::from(([192, 168, 1, 1], 8080));
        let addr = parse_socket_addr(&"https://192.168.1.1:8080".parse().unwrap()).unwrap();
        assert_eq!(addr, expected_addr);

        // URL with hostname and port; for now, this is not supported
        let addr = parse_socket_addr(&"http://example.com:80".parse().unwrap()).unwrap_err();
        assert_eq!(
            addr,
            "Failed to parse socket address: invalid socket address syntax"
        );
    }

    #[test(tokio::test)]
    async fn test_router() {
        let (tx, rx) = async_channel::unbounded();
        let csrf = CsrfToken::new_random();

        let uri = Uri::builder()
            .path_and_query(format!("/?code=code&state={}", csrf.secret()))
            .build()
            .unwrap();

        let response = app(config(csrf, tx))
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(
            String::from_utf8(bytes.to_vec()).unwrap(),
            r#"
    <html>
    <head><title>Authorization Successful</title></head>
    <body>
        <h1>✓ Authorization Successful</h1>
        <p>You can now safely close this window.</p>
    </body>
    </html>
    "#
        );
        assert_eq!(rx.recv().await.unwrap(), "code");
    }

    #[test(tokio::test)]
    #[ignore = "Testing manually for browser verification. Should be --nocapture"]
    async fn test_local_server_messenger() {
        let context = AuthContext {
            auth_url: "https://127.0.0.1:8081/?state=CSRF&code=code"
                .parse()
                .unwrap(),
            csrf: CsrfToken::new("CSRF".to_string()),
            redirect_url: "https://127.0.0.1:8081".parse().unwrap(),
        };

        let certs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs");
        let mut messenger = LocalServerMessenger::new(&certs_dir).await.unwrap();

        messenger.with_context(context).await.unwrap();
        messenger.send_auth_message().await.unwrap();

        assert_eq!("code", messenger.receive_auth_message().await.unwrap());
    }
}
