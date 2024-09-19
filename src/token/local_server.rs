use axum::{
    extract::{FromRef, Query, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use oauth2::CsrfToken;
use std::net::SocketAddr;
use std::path::PathBuf;
use url::Url;

pub(super) async fn local_server(
    csrf: CsrfToken,
    certs_dir: PathBuf,
    redirect_url: &Url,
) -> String {
    let (tx, rx) = async_channel::unbounded();

    let app_state = AppState { csrf, tx };

    // configure certificate and private key used by https
    let config = RustlsConfig::from_pem_file(certs_dir.join("cert.pem"), certs_dir.join("key.pem"))
        .await
        .expect("certs setting ok");

    // Derive SocketAddr from redirect_url
    let addr = parse_socket_addr(redirect_url).expect("SocketAddr");

    tokio::spawn(axum_server::bind_rustls(addr, config).serve(app(app_state).into_make_service()));

    rx.recv().await.expect("receive code")
}

fn app(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_code))
        .with_state(app_state)
}

#[derive(Clone, FromRef)]
struct AppState {
    csrf: CsrfToken,
    tx: async_channel::Sender<String>,
}

async fn get_code(
    Query(query): Query<super::auth::AuthRequest>,
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
}
