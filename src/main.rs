mod websocket;
mod exchanges;
mod rest;

use tokio;
use exchanges::binance_ws::BinanceWebSocket;
use websocket::websocket_handler::WebSocketHandler;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Start Binance WebSocket connection by calling the trait method
    tokio::spawn(async {
        let _ = BinanceWebSocket::start().await;
    });

    // Sleep to keep the application running while WebSocket is active
    println!("Binance WebSocket connection started...");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
