use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

async fn authenticate(api_key: &str, api_secret: &str) {
    // Generate expires
    let start = SystemTime::now();
    let expires = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() + 1000; // Adding 1 second to the current time

    // Prepare signature message
    let message = format!("GET/realtime{}", expires);

    // Create HMAC-SHA256 signature
    let mut mac = HmacSha256::new_varkey(api_secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(message.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    // WebSocket URL (replace with actual URL)
    let url = Url::parse("wss://stream.bybit.com/realtime").unwrap();

    // Connect to WebSocket server
    let (mut socket, _) = connect_async(url).await.expect("Failed to connect");

    // Authenticate with API
    let auth_message = json!({
        "op": "auth",
        "args": [api_key, expires.to_string(), signature]
    });

    // Send authentication message
    socket
        .send(Message::Text(auth_message.to_string()))
        .await
        .expect("Failed to send message");
}

// Usage
#[tokio::main]
async fn main() {
    let api_key = "your_api_key";
    let api_secret = "your_api_secret";

    authenticate(api_key, api_secret).await;
}