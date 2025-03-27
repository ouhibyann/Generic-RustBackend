use crate::websocket::websocket_handler::WebSocketHandler;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use url::Url;
use http::uri::Uri;

pub struct BinanceWebSocket;

impl WebSocketHandler for BinanceWebSocket {
    fn start() -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            // Binance WebSocket endpoint
            let url = Url::parse("wss://stream.binance.com:9443/ws/btcusdt@depth").unwrap();
            let uri: Uri = url.as_str().parse().unwrap();

            // Establish WebSocket connection to Binance
            let (ws_stream, _) = connect_async(uri).await.expect("Failed to connect to Binance WebSocket");
            let (mut write, mut read) = ws_stream.split();

            while let Some(message) = read.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        println!("Received: {}", text);
                    }
                    Ok(Message::Ping(ping)) => {
                        // Respond to WebSocket ping messages with a pong to maintain connection
                        write.send(Message::Pong(ping)).await.expect("Failed to send Pong");
                    }
                    Ok(Message::Close(_)) => {
                        // Handle WebSocket closure
                        println!("WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        // Log any errors that occur
                        eprintln!("WebSocket error: {:?}", e);
                    }
                    _ => {}
                }
            }
        })
    }
}