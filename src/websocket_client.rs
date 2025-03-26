use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use tokio::time::{sleep, Duration};
use url::Url;
use crate::utils::merge_strings;

/// Connects to the WebSocket server and listens for messages
pub async fn start_websocket_client() {
    let url = Url::parse("ws://127.0.0.1:8080").expect("Invalid WebSocket URL");

    loop {
        match connect_async(url).await {
            Ok((mut ws_stream, _)) => {
                println!("✅ Connected to WebSocket server!");

                while let Some(msg) = ws_stream.next().await {
                    if let Ok(Message::Text(received)) = msg {
                        println!("📥 Received from server: {}", received);

                        // Merge transcriptions
                        let merged_result = merge_strings(&received, ""); // Placeholder for additional text
                        println!("🔄 Merged Result: {}", merged_result);

                        // Send back to server
                        ws_stream.send(Message::Text(merged_result)).await.ok();
                    }
                }
            }
            Err(err) => {
                println!("❌ Connection failed: {}", err);
                println!("🔄 Retrying in 3 seconds...");
                sleep(Duration::from_secs(3)).await;
            }
        }
    }
}
