// src/main.rs
mod exchanges;
mod rest;
mod websocket;

use std::env;
use actix_web::{App, HttpServer};
use exchanges::binance_ws::BinanceWebSocket;
use websocket::websocket_handler::WebSocketHandler;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Read the first command-line argument to decide which mode to run.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <mode>", args[0]);
        eprintln!("Modes: rest, ws");
        return Ok(());
    }

    match args[1].as_str() {
        "rest" => run_rest().await,
        "ws" => run_ws().await,
        mode => {
            eprintln!("Unknown mode: {}. Use 'rest' or 'ws'", mode);
            Ok(())
        }
    }
}

/// Run Actix Web server to expose REST endpoints.
async fn run_rest() -> std::io::Result<()> {
    println!("Starting REST server on http://127.0.0.1:8080...");
    HttpServer::new(|| {
        App::new()
            .configure(rest::init_routes) // Register REST endpoints (e.g., /kraken/mid_price)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

/// Run the Binance WebSocket integration.
async fn run_ws() -> std::io::Result<()> {
    println!("Starting Binance WebSocket connection...");
    // Spawn the WebSocket task.
    tokio::spawn(async {
        let _ = BinanceWebSocket::start().await;
    });

    // Keep the process alive to allow the WebSocket to work.
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
