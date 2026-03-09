use std::collections::HashMap;

use serde::Serialize;

use super::{Command, Service};

#[derive(Debug, Clone, Serialize)]
pub struct StreamerRequest {
    pub requests: Vec<Request>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub service: Service,
    pub requestid: String,
    pub command: Command,
    #[serde(rename = "SchwabClientCustomerId")]
    pub schwab_client_customer_id: String,
    #[serde(rename = "SchwabClientCorrelId")]
    pub schwab_client_correl_id: String,
    pub parameters: HashMap<String, String>,
}
