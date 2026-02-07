//! A messenger that uses standard input/output.

use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::instrument;

use super::{AuthContext, ChannelMessenger};
use crate::error::Error;

#[derive(Debug)]
pub struct CompoundMessenger<CM0: ChannelMessenger, CM1: ChannelMessenger> {
    select: AtomicUsize,
    default: CM0,
    other: CM1,
}

impl<CM0: ChannelMessenger, CM1: ChannelMessenger> CompoundMessenger<CM0, CM1> {
    pub fn new(default: CM0, other: CM1) -> Self {
        Self {
            select: AtomicUsize::new(0),
            default,
            other,
        }
    }
}

impl<CM0: ChannelMessenger, CM1: ChannelMessenger> ChannelMessenger
    for CompoundMessenger<CM0, CM1>
{
    #[instrument(skip(self, context), fields(redirect_url = %context.redirect_url))]
    async fn with_context(&mut self, context: AuthContext) -> Result<(), Error> {
        tracing::debug!("configuring compound messenger with auth context");

        self.default
            .with_context(context.clone())
            .await
            .inspect_err(|e| {
                tracing::error!(error = %e, "failed to configure default messenger");
            })?;

        self.other.with_context(context).await.inspect_err(|e| {
            tracing::error!(error = %e, "failed to configure secondary messenger");
        })?;

        tracing::info!("compound messenger configured successfully");
        Ok(())
    }

    #[instrument(skip(self))]
    async fn send_auth_message(&self) -> Result<(), Error> {
        loop {
            let current_index = self.select.load(Ordering::Acquire);

            let result = match current_index {
                0 => {
                    tracing::debug!("attempting authentication via default messenger");
                    self.default.send_auth_message().await
                }
                1 => {
                    tracing::debug!("attempting authentication via secondary messenger");
                    self.other.send_auth_message().await
                }
                _ => {
                    tracing::error!(
                        index = %current_index,
                        "no messengers available for authentication"
                    );

                    return Err(Error::ChannelMessenger(
                        "No Messengers available to send".to_string(),
                    ));
                }
            };

            match result {
                Ok(()) => {
                    tracing::info!(index = %current_index, "authentication message sent successfully");
                    return Ok(());
                }
                Err(e) => {
                    tracing::warn!(
                        index = %current_index,
                        error = %e,
                        next_index = %(current_index + 1),
                        "messenger authentication failed; switching to next provider"
                    );

                    self.select.fetch_add(1, Ordering::AcqRel);
                }
            }
        }
    }

    #[instrument(skip(self))]
    async fn receive_auth_message(&self) -> Result<String, Error> {
        let current_index = self.select.load(Ordering::Acquire);
        tracing::debug!(index = %current_index, "receiving auth message from selected messenger");

        match current_index {
            0 => {
                tracing::debug!("receiving from default messenger");
                self.default.receive_auth_message().await
            }
            1 => {
                tracing::debug!("receiving from secondary messenger");
                self.other.receive_auth_message().await
            }
            _ => {
                tracing::error!(index = %current_index, "invalid messenger index");
                Err(Error::ChannelMessenger(
                    "No active messenger to receive from".to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use oauth2::CsrfToken;
    use std::path::PathBuf;
    use test_log::test;

    use crate::token::channel_messenger::{
        local_server::LocalServerMessenger, stdio_messenger::StdioMessenger,
    };

    use super::*;

    #[test(tokio::test)]
    #[ignore = "Testing manually for compound verification. Should be --nocapture"]
    async fn test_compound_messenger() {
        let context = AuthContext {
            auth_url: "https://127.0.0.1:8081/?state=CSRF&code=code"
                .parse()
                .unwrap(),
            csrf: CsrfToken::new("CSRF".to_string()),
            redirect_url: "https://127.0.0.1:8081".parse().unwrap(),
        };

        let certs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/certs");
        let mut messenger = CompoundMessenger::new(
            LocalServerMessenger::new(&certs_dir).await.unwrap(),
            StdioMessenger::new(),
        );

        messenger.with_context(context).await.unwrap();
        messenger.send_auth_message().await.unwrap();

        // if in stdio, you should input https://127.0.0.1:8081/?state=CSRF&code=code
        assert_eq!("code", messenger.receive_auth_message().await.unwrap());
    }
}
