use axum::{
    extract::{FromRef, Query, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use oauth2::CsrfToken;
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::PathBuf;

pub(super) async fn local_server(csrf: CsrfToken) -> String {
    let (tx, rx) = async_channel::unbounded();

    let app_state = AppState { csrf, tx };

    // configure certificate and private key used by https
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("certs")
            .join("key.pem"),
    )
    .await
    .expect("certs setting ok");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthRequest {
    code: String,
    state: String,
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
        "TDAmeritrade returned the following code:\n{}\nYou can now safely close this browser window.",
        &query.code
    );

    tx.send(query.code).await.expect("send code");

    content
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode, Uri},
    };
    use tower::ServiceExt; // for `oneshot` and `ready`

    fn config(csrf: CsrfToken, tx: async_channel::Sender<String>) -> AppState {
        AppState { csrf, tx }
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
            "TDAmeritrade returned the following code:\ncode\nYou can now safely close this browser window."
        );
        assert_eq!(rx.recv().await.unwrap(), "code");
    }
}
