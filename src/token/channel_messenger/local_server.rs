use async_channel::Receiver;
use axum::{
    extract::{FromRef, Query, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use oauth2::CsrfToken;
use std::{net::SocketAddr, path::Path, result::Result};
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
    /// # Panics
    ///
    /// Will panic without cert.pem and key.pem
    pub async fn new(certs_dir: &Path) -> Self {
        Self {
            config: RustlsConfig::from_pem_file(
                certs_dir.join("cert.pem"),
                certs_dir.join("key.pem"),
            )
            .await
            .expect("certs setting ok"),

            addr: None,
            rx: None,
            app_state: None,
            auth_url: None,
        }
    }
}

impl ChannelMessenger for LocalServerMessenger {
    async fn with_context(&mut self, context: AuthContext) -> Result<(), Error> {
        let (tx, rx) = async_channel::unbounded();
        let csrf = context.csrf.as_ref().expect("csrf").clone();
        let redirect_uri = context.redirect_url.as_ref().expect("redirect_url");

        self.app_state = Some(AppState { csrf, tx });
        self.rx = Some(rx);
        self.auth_url = Some(context.auth_url.as_ref().expect("url").clone());
        self.addr = Some(parse_socket_addr(redirect_uri).expect("SocketAddr"));

        Ok(())
    }

    async fn send_auth_message(&self) -> Result<(), Error> {
        open::that(self.auth_url.as_ref().expect("auth_url").as_ref())?;

        Ok(())
    }

    async fn receive_auth_message(&self) -> Result<String, Error> {
        let service = app(self.app_state.as_ref().expect("app_state").clone()).into_make_service();
        tokio::spawn(
            axum_server::bind_rustls(
                *self.addr.as_ref().expect("SocketAddr"),
                self.config.clone(),
            )
            .serve(service),
        );

        let code = self
            .rx
            .as_ref()
            .expect("rx")
            .recv()
            .await
            .expect("receive code");
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

async fn get_code(
    Query(query): Query<AuthRequest>,
    State(csrf): State<CsrfToken>,
    State(tx): State<async_channel::Sender<String>>,
) -> impl IntoResponse {
    if &query.state != csrf.secret() {
        return "CSRF check error".to_string();
    }

    let content = format!(
        "Schwab returned the following code:\n{}\nYou can now safely close this browser window.",
        &query.code
    );

    tx.send(query.code).await.expect("send code");

    content
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
    use super::*;

    use axum::{
        body::Body,
        http::{Request, StatusCode, Uri},
    };
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;
    use tower::ServiceExt; // for `oneshot` and `ready`

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

    #[tokio::test]
    async fn test_local_server() {
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
            "Schwab returned the following code:\ncode\nYou can now safely close this browser window."
        );
        assert_eq!(rx.recv().await.unwrap(), "code");
    }

    #[tokio::test]
    #[ignore = "If the test is performed manually on Linux, it may fail for HTTPS."]
    async fn test_receive_auth_message() {
        let redirect_url = "https://127.0.0.1:8081";

        let auth_code = tokio::spawn(async {
            let certs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs");

            let mut messenger = LocalServerMessenger::new(&certs_dir).await;

            let context = AuthContext {
                auth_url: Some(redirect_url.parse().unwrap()),
                csrf: Some(CsrfToken::new("CSRF".to_string())),
                redirect_url: Some(redirect_url.parse().unwrap()),
            };
            messenger.with_context(context).await.unwrap();

            messenger.receive_auth_message().await.unwrap()
        });

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        let body = client
            .get(format!("{redirect_url}/?state=CSRF&code=code"))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        assert_eq!(auth_code.await.unwrap(), "code");
        assert_eq!(body, "Schwab returned the following code:\ncode\nYou can now safely close this browser window.");
    }
}
