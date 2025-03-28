use crate::websocket::websocket_handler::WebSocketHandler;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use url::Url;
use http::uri::Uri;
use serde_json::Value;

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
                        // Parse the incoming JSON message from Binance
                        if let Ok(json) = serde_json::from_str::<Value>(&text) {
                            // Extract bids and asks arrays from the JSON message
                            // See: https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams
                            if let (Some(asks), Some(bids)) = (json["a"].as_array(), json["b"].as_array()) {
                                if let (Some(best_ask), Some(best_bid)) = (asks.first(), bids.first()) {
                                    if let (Some(ask_price), Some(bid_price)) = (best_ask[0].as_str(), best_bid[0].as_str()) {
                                        let ask_price: f64 = ask_price.parse().unwrap_or(0.0);
                                        let bid_price: f64 = bid_price.parse().unwrap_or(0.0);

                                        let mid_price = (ask_price + bid_price) / 2.0;
                                        println!("Highest Bid: {}, Lowest Ask: {}, Mid Price: {}", bid_price, ask_price, mid_price);
                                    }
                                }
                            }
                        }
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