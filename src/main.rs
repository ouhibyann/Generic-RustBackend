// src/main.rs
mod config;
mod exchanges {
    pub mod orderbook_provider;
    pub mod kraken_rest_api;
    // pub mod binance_ws; // Other modules can be added here.
}
mod rest;

use actix_web::{App, HttpServer};
use config::load_config;
use exchanges::kraken_rest_api::KrakenProvider;
use exchanges::orderbook_provider::OrderBookProvider;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Determine mode from command-line arguments: "rest" or "ws".
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <mode>", args[0]);
        eprintln!("Modes: rest, ws");
        return Ok(());
    }

    // Load configuration.
    let config = load_config().expect("Failed to load configuration");
    let kraken_config = config.kraken;

    // Create a Kraken provider using the configuration.
    let kraken_provider = KrakenProvider {
        trading_pair: kraken_config.trading_pair,
        endpoint: kraken_config.endpoint,
    };

    // Wrap the provider in an Arc so it can be shared.
    let provider_arc: Arc<dyn OrderBookProvider + Send + Sync> = Arc::new(kraken_provider);

    match args[1].as_str() {
        "rest" => run_rest(provider_arc).await,
        "ws" => run_ws().await,
        mode => {
            eprintln!("Unknown mode: {}. Use 'rest' or 'ws'", mode);
            Ok(())
        }
    }
}

/// Run Actix Web server to expose REST endpoints.
async fn run_rest(provider: Arc<dyn OrderBookProvider + Send + Sync>) -> std::io::Result<()> {
    println!("Starting REST server on http://127.0.0.1:8080...");
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(provider.clone()))
            .configure(rest::init_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

/// Run the Binance WebSocket integration.
async fn run_ws() -> std::io::Result<()> {
    println!("Starting WebSocket mode...");
    // For now, we simply keep the process alive.
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
