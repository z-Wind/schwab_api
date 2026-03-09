use std::path::PathBuf;

use dotenv;
use reqwest::Client;
use schwab_api::Api;
use schwab_api::model::trader::user_preference::UserPreferences;
use schwab_api::streaming::StreamingClient;
use schwab_api::token::{TokenChecker, Tokener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let key = std::env::var("SCHWAB_APP_KEY").expect("SCHWAB_APP_KEY not set");
    let secret = std::env::var("SCHWAB_APP_SECRET").expect("SCHWAB_APP_SECRET not set");
    let callback_url =
        std::env::var("CALLBACK_URL").unwrap_or_else(|_| "https://127.0.0.1:8080".to_string());
    let creds_dir = std::env::var("CREDS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(".creds"));
    let certs_dir = std::env::var("CERTS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(".certs"));

    let client = Client::new();
    let token_checker = TokenChecker::new_with_local_server(
        creds_dir,
        key,
        secret,
        callback_url,
        certs_dir,
        client.clone(),
    )
    .await?;

    let api = Api::new(token_checker, client).await?;

    // Fetch streaming connection info from user preferences.
    let prefs = api.get_user_preference().await?.send().await?;
    let streamer_info = match prefs {
        UserPreferences::One(p) => p.streamer_info.into_iter().next(),
        UserPreferences::Mutiple(_) => panic!(),
    }
    .expect("no streamer info in user preferences");

    let access_token = api.tokener.get_access_token().await?;

    let mut stream = StreamingClient::connect(streamer_info, access_token).await?;
    stream.subscribe_account_activity().await?;

    println!("Connected. Listening for account activity...");

    while let Some(result) = stream.next_account_activity().await {
        match result {
            Ok(activities) => {
                for a in activities {
                    serde_json::to_string_pretty(&a)
                        .map(|s| println!("Received account activity:\n{s}"))
                        .unwrap_or_else(|e| eprintln!("Failed to serialize activity: {e}"));
                    println!("\n\n");
                }
            }
            Err(e) => eprintln!("error: {e}"),
        }
    }

    println!("Stream closed.");
    Ok(())
}
