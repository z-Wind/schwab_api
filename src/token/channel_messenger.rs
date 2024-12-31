pub mod local_server;
pub mod stdio_messenger;

use oauth2::CsrfToken;
use url::Url;

use crate::Error;

#[derive(Debug)]
pub struct AuthContext {
    pub auth_url: Option<Url>,
    pub csrf: Option<CsrfToken>,
    pub redirect_url: Option<Url>,
}

/// A trait for sending and receiving messages through a channel.
///
/// Implementors of this trait provide a way to send messages to a recipient
/// and receive responses.
pub trait ChannelMessenger: Sync {
    fn with_context(
        &mut self,
        context: AuthContext,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;

    /// Transmits a message through the `tx` channel.
    ///
    /// # Arguments
    ///
    /// * `message`: The message to be sent.
    fn send_auth_message(&self) -> impl std::future::Future<Output = Result<(), Error>> + Send;

    /// Receives a message from the `rx` channel.
    ///
    /// # Returns
    ///
    /// The received message as a `String`.
    fn receive_auth_message(
        &self,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;
}
