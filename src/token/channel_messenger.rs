pub mod compound_messenger;
pub mod local_server;
pub mod stdio_messenger;

use oauth2::CsrfToken;
use url::Url;

pub use compound_messenger::CompoundMessenger;
pub use local_server::LocalServerMessenger;
pub use stdio_messenger::StdioMessenger;

use crate::Error;

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub auth_url: Url,
    pub csrf: CsrfToken,
    pub redirect_url: Url,
}

/// A trait for sending and receiving messages through a channel.
///
/// Implementors of this trait provide a way to send messages to a recipient
/// and receive responses.
pub trait ChannelMessenger: Sync + Send {
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
