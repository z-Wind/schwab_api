use std::collections::HashMap;

use serde::Deserialize;

use super::{account_activity::AccountActivity, command::Command, service::Service};
use crate::{Error, streaming::model::AccountActivityEvent};

// ─────────────────────────────────────────────────────────────
// Public message types
// ─────────────────────────────────────────────────────────────

/// Individual parsed messages extracted from a [`RawStreamerFrame`].
#[derive(Debug, Clone)]
pub enum StreamerMessage {
    Notify(HeartbeatData),
    Response(ResponseMessage),
    Data(DataMessage),
    Snapshot(DataMessage),
}

// ─────────────────────────────────────────────────────────────
// Internal frame types
// ─────────────────────────────────────────────────────────────

/// Every WebSocket frame from the server deserialises into this.
///
/// A single frame may carry *multiple* top-level keys (e.g. both `data` and
/// `notify`), so we use a plain struct rather than an `untagged` enum.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RawStreamerFrame {
    #[serde(default)]
    pub notify: Vec<HeartbeatData>,
    #[serde(default)]
    pub response: Vec<ResponseMessage>,
    #[serde(default)]
    pub data: Vec<DataMessage>,
    #[serde(default)]
    pub snapshot: Vec<DataMessage>,
}

impl RawStreamerFrame {
    pub fn messages(&self) -> impl Iterator<Item = RawStreamerMessage> + '_ {
        self.notify
            .iter()
            .map(|n| RawStreamerMessage::Notify(n.clone()))
            .chain(
                self.response
                    .iter()
                    .map(|r| RawStreamerMessage::Response(r.clone())),
            )
            .chain(
                self.data
                    .iter()
                    .map(|d| RawStreamerMessage::Data(d.clone())),
            )
            .chain(
                self.snapshot
                    .iter()
                    .map(|s| RawStreamerMessage::Snapshot(s.clone())),
            )
    }
}

#[derive(Debug, Clone)]
pub enum RawStreamerMessage {
    Notify(HeartbeatData),
    Response(ResponseMessage),
    Data(DataMessage),
    Snapshot(DataMessage),
}

impl RawStreamerMessage {
    pub fn account_activities(&self) -> Result<Vec<AccountActivity>, Error> {
        match self {
            Self::Data(msg) | Self::Snapshot(msg) => msg.account_activities(),
            _ => Ok(vec![]),
        }
    }
}

// ─────────────────────────────────────────────────────────────
// Heartbeat
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct HeartbeatData {
    pub heartbeat: String,
}

// ─────────────────────────────────────────────────────────────
// Response (reply to a command)
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseMessage {
    pub service: String,
    pub command: Command,
    pub requestid: String,
    pub timestamp: i64,
    pub content: ResponseContent,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseContent {
    pub code: i32,
    pub msg: String,
}

impl ResponseContent {
    pub fn is_success(&self) -> bool {
        self.code == 0
    }
}

// ─────────────────────────────────────────────────────────────
// Data / snapshot (streaming market data)
// ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct DataMessage {
    pub service: Service,
    pub timestamp: i64,
    pub command: Command,
    pub content: Vec<HashMap<String, serde_json::Value>>,
}

impl DataMessage {
    /// Parse content items as [`AccountActivity`] records.
    /// Returns an empty `Vec` if the service is not `ACCT_ACTIVITY`.
    pub(super) fn account_activities(&self) -> Result<Vec<AccountActivity>, Error> {
        if !matches!(self.service, Service::AccountActivity) {
            return Ok(vec![]);
        }
        // dbg!(&self);

        self.content
            .iter()
            .map(|entry| {
                let key = entry
                    .get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| Error::Streaming("missing key".into()))?
                    .to_string();

                let account_number = entry
                    .get("1")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| Error::Streaming("missing account number".into()))?
                    .to_string();

                let raw_type = entry
                    .get("2")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| Error::Streaming("missing activity type".into()))?;

                let data = entry
                    .get("3")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                Ok(AccountActivity {
                    key,
                    account_number,
                    event: AccountActivityEvent::from_raw(raw_type, data).map_err(|e| {
                        Error::Streaming(format!("Failed to parse {raw_type}: {e}. Self: {self:?}"))
                    })?,
                })
            })
            .collect()
    }
}
