//! **This is not an official API or a stable recreation of the Charles Schwab API. Functionality may change due to updates made by Schwab.**
//!
//! ## Overview
//! Currently, only supports the API of individual developers.
//!
//! ## Prerequisites
//! 1. To use the API, you need to apply for access on the [Charles Schwab Developer Portal](https://developer.schwab.com/home). Upon approval, you will receive the necessary Key and Secret.
//! 2. (Option) Create a self-signed certificate
//!     ```bash
//!     openssl req -newkey rsa:4096 -new -nodes -x509 -days 3650 -keyout key.pem -out cert.pem
//!     ```
//!
//! ## Example
//! ```no_run
//! use std::path::PathBuf;
//! use reqwest::Client;
//!
//! use schwab_api::api;
//! use schwab_api::token::TokenChecker;
//!
//! #[tokio::main]
//! async fn main() {
//!     let key = "your_app_key".to_string();
//!     let secret = "your_secret".to_string();
//!     let callback_url = "https://127.0.0.1:8080".to_string();
//!     let path = dirs::home_dir()
//!         .expect("home dir")
//!         .join(".credentials")
//!         .join("Schwab-rust.json");
//!     let certs_dir = PathBuf::from("your_certs_dir");
//!
//!     let client = Client::new();
//!     let token_checker = TokenChecker::new(path, key, secret, callback_url, certs_dir, client.clone())
//!         .await
//!         .unwrap();
//!
//!     let api = api::Api::new(token_checker, client).await.unwrap();
//!
//!     let req = api.get_quote("VTI".to_string()).await.unwrap();
//!     let rsp = req.send().await.unwrap();
//!     println!("{:?}", rsp);
//! }
//! ```
//!
//! **Disclaimer:** *This is an unofficial API wrapper for Schwab. It is not endorsed by or affiliated with Schwab or any associated organization. Before using this package, make sure to read and understand the terms of service of the underlying API. The authors of this package accept no responsibility for any damage that might stem from its use. Refer to the LICENSE file for more details.*

#![forbid(unsafe_code)]
#![warn(
clippy::pedantic,
missing_copy_implementations,
missing_debug_implementations,
//missing_docs,
rustdoc::broken_intra_doc_links,
trivial_numeric_casts,
unused_allocation
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::implicit_hasher,
    clippy::similar_names,
    clippy::module_name_repetitions
)]

pub mod api;
pub mod error;
pub mod model;
pub mod token;

pub use api::Api;
pub use error::Error;
