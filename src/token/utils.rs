use std::path::PathBuf;

use async_trait::async_trait;
use oauth2::CsrfToken;
use url::Url;

/// A trait for sending and receiving messages through a channel.
///
/// Implementors of this trait provide a way to send messages to a recipient
/// and receive responses.
#[derive(Debug)]
pub struct AuthContext {
    pub url: Option<Url>,
    pub csrf: Option<CsrfToken>,
    pub redirect_url: Option<Url>,
    pub certs_dir: Option<PathBuf>,
}

#[async_trait]
pub trait ChannelMessenger: std::fmt::Debug {
    async fn with_context(&self, context: AuthContext) -> Result<(), Box<dyn std::error::Error>>;

    /// Transmits a message through the `tx` channel.
    ///
    /// # Arguments
    ///
    /// * `message`: The message to be sent.
    async fn send_auth_message(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// Receives a message from the `rx` channel.
    ///
    /// # Returns
    ///
    /// The received message as a `String`.
    async fn receive_auth_message(&self) -> Result<String, Box<dyn std::error::Error>>;
}
