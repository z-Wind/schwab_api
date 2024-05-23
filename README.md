# Schwab API
[![CI](https://github.com/z-Wind/schwab_api/actions/workflows/ci.yml/badge.svg)](https://github.com/z-Wind/schwab_api/actions/workflows/ci.yml)

**This is not an official API or a stable recreation of the Charles Schwab API. Functionality may change due to updates made by Schwab.**

## Overview
Currently, the API only supports individual developers.

## Prerequisites
1. To use the API, you need to apply for access on the [Charles Schwab Developer Portal](https://developer.schwab.com/home). Upon approval, you will receive the necessary Key and Secret.
2. Create a self-signed certificate
    ```
    openssl req -newkey rsa:4096 -new -nodes -x509 -days 3650 -keyout key.pem -out cert.pem
    ```

## Example
```rust
use std::path::PathBuf;

use schwab_api::api;
use schwab_api::token::TokenChecker;

#[tokio::main]
async fn main() {
    let key = "your_app_key".to_string();
    let secret = "your_secret".to_string();
    let callback_url = "https://127.0.0.1:8080".to_string();
    let path = dirs::home_dir()
        .expect("home dir")
        .join(".credentials")
        .join("Schwab-rust.json");
    let certs_dir = PathBuf::from("your_certs_dir");
    let token_checker = TokenChecker::new(path, key, secret, callback_url, certs_dir)
        .await
        .unwrap();

    let api = api::Api::new(token_checker);

    let req = api.get_quote("VTI".to_string()).await.unwrap();
    let rsp = req.send().await.unwrap();
    println!("{:?}", rsp);
}
```

## Installation
```toml
# Cargo.toml
[dependencies]
schwab_api = "0.1"
```

## Reference
* [GitHub - alexgolec/schwab-py: Unofficial API wrapper for the upcoming Schwab HTTP API](https://github.com/alexgolec/schwab-py/tree/main)

**Disclaimer:** *This is an unofficial API wrapper for Schwab. It is not endorsed by or affiliated with Schwab or any associated organization. Before using this package, make sure to read and understand the terms of service of the underlying API. The authors of this package accept no responsibility for any damage that might stem from its use. Refer to the LICENSE file for more details.*
