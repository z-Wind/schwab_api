//! WebSocket streaming client for real-time account activity.

pub mod model;

use std::collections::HashMap;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use model::RawStreamerFrame;

use crate::{
    error::Error,
    model::trader::user_preference::StreamerInfo,
    streaming::model::{AccountActivity, Command, Request, Service, StreamerRequest},
};

type WsStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

/// A connected and authenticated streaming client.
pub struct StreamingClient {
    ws: WsStream,
    streamer_info: StreamerInfo,
    request_counter: u64,
}

impl std::fmt::Debug for StreamingClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamingClient")
            .field("streamer_info", &self.streamer_info)
            .field("request_counter", &self.request_counter)
            .finish_non_exhaustive()
    }
}

impl StreamingClient {
    /// Connect to Schwab's streaming server and log in.
    pub async fn connect(streamer_info: StreamerInfo, access_token: String) -> Result<Self, Error> {
        let (ws, _) = connect_async(&streamer_info.streamer_socket_url)
            .await
            .map_err(|e| Error::Streaming(e.to_string()))?;

        let mut client = Self {
            ws,
            streamer_info,
            request_counter: 0,
        };

        client.login(access_token).await?;
        Ok(client)
    }

    async fn login(&mut self, access_token: String) -> Result<(), Error> {
        let id = self.next_id();
        let mut params = HashMap::new();
        params.insert("Authorization".to_string(), access_token);
        params.insert(
            "SchwabClientChannel".to_string(),
            self.streamer_info.schwab_client_channel.clone(),
        );
        params.insert(
            "SchwabClientFunctionId".to_string(),
            self.streamer_info.schwab_client_function_id.clone(),
        );

        self.send_request(StreamerRequest {
            requests: vec![Request {
                service: Service::Admin,
                requestid: id,
                command: Command::Login,
                schwab_client_customer_id: self.streamer_info.schwab_client_customer_id.clone(),
                schwab_client_correl_id: self.streamer_info.schwab_client_correl_id.clone(),
                parameters: params,
            }],
        })
        .await?;

        // Wait for the login response to confirm success.
        while let Some(msg) = self.ws.next().await {
            let text = match msg.map_err(|e| Error::Streaming(e.to_string()))? {
                Message::Text(t) => t,
                _ => continue,
            };
            let raw_frame: RawStreamerFrame = serde_json::from_str(&text)?;

            for response in raw_frame.response {
                match response.command {
                    Command::Login => return Ok(()),
                    _ => Err(Error::Streaming(
                        "unexpected message type before login response".to_string(),
                    ))?,
                }
            }
        }

        Err(Error::Streaming(
            "connection closed before login response".to_string(),
        ))
    }

    /// Subscribe to account activity events.
    pub async fn subscribe_account_activity(&mut self) -> Result<(), Error> {
        let id = self.next_id();
        let mut params = HashMap::new();
        params.insert(
            "keys".to_string(),
            self.streamer_info.schwab_client_customer_id.clone(),
        );
        params.insert("fields".to_string(), "0,1,2,3".to_string());

        self.send_request(StreamerRequest {
            requests: vec![Request {
                service: Service::AccountActivity,
                requestid: id,
                command: Command::Subscriptions,
                schwab_client_customer_id: self.streamer_info.schwab_client_customer_id.clone(),
                schwab_client_correl_id: self.streamer_info.schwab_client_correl_id.clone(),
                parameters: params,
            }],
        })
        .await
    }

    /// Returns the next batch of account activity events.
    pub async fn next_account_activity(&mut self) -> Option<Result<Vec<AccountActivity>, Error>> {
        loop {
            let msg = self.ws.next().await?;

            let text = match msg {
                Ok(Message::Text(t)) => t,
                Ok(_) => return Some(Err(Error::Streaming("Unexpected message type".to_string()))),
                Err(e) => return Some(Err(Error::Streaming(e.to_string()))),
            };

            let raw_frame: RawStreamerFrame = match serde_json::from_str(&text) {
                Ok(r) => r,
                Err(e) => {
                    return Some(Err(Error::Streaming(format!(
                        "Failed to parse streaming message: {}",
                        e
                    ))));
                }
            };

            let mut activities = Vec::new();
            for msg in raw_frame.messages() {
                match msg.account_activities() {
                    Ok(acts) => activities.extend(acts.clone()),
                    Err(e) => return Some(Err(e)),
                }
            }
            // dbg!(&activities);

            if !activities.is_empty() {
                return Some(Ok(activities));
            }
        }
    }

    async fn send_request(&mut self, req: StreamerRequest) -> Result<(), Error> {
        let json = serde_json::to_string(&req)?;
        self.ws
            .send(Message::Text(json))
            .await
            .map_err(|e| Error::Streaming(e.to_string()))
    }

    fn next_id(&mut self) -> String {
        let id = self.request_counter;
        self.request_counter += 1;
        id.to_string()
    }
}
