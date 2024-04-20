# Schwab API
[![CI](https://github.com/z-Wind/schwab_api/actions/workflows/ci.yml/badge.svg)](https://github.com/z-Wind/schwab_api/actions/workflows/ci.yml)

**This is not an official API or a stable recreation of the Charles Schwab API. Functionality may change due to updates made by Schwab.**

## Overview
Currently, the API only supports individual developers.

## Prerequisites
To use the API, you need to apply for access on the [Charles Schwab Developer Portal](https://developer.schwab.com/home). Upon approval, you will receive the necessary Key and Secret.

## Example
```rust
use schwab_api::api;

#[tokio::main]
async fn main() {
    let key = "your_api_key".to_string();
    let secret = "your_secret".to_string();
    let price = api::API::new(key, secret)
        .await
        .unwrap()
        .get_quote("VTI".to_string())
        .await
        .unwrap()
        .send()
        .await
        .unwrap();
    println!("{:?}", price);
}

```

## Installation
```toml
# Cargo.toml
[dependencies]
schwab_api = "0.1"
```

**Disclaimer:** *This is an unofficial API wrapper for Schwab. It is not endorsed by or affiliated with Schwab or any associated organization. Before using this package, make sure to read and understand the terms of service of the underlying API. The authors of this package accept no responsibility for any damage that might stem from its use. Refer to the LICENSE file for more details.*