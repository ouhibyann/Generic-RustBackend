// src/main.rs
mod config;
mod exchanges {
    pub mod orderbook_provider;
    pub mod kraken_rest_api;
    pub mod huobi_rest_api;
    pub mod binance_ws;
}
mod rest;
mod websocket;
use actix_web::{App, HttpServer};
use config::load_config;
use exchanges::kraken_rest_api::KrakenProvider;
use exchanges::huobi_rest_api::HuobiProvider;
use std::env;
use std::sync::Arc;
use rest::{HuobiData, KrakenData};

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
    let huobi_config = config.huobi;

    // Create providers.
    let kraken_provider = KrakenProvider {
        trading_pair: kraken_config.trading_pair,
        endpoint: kraken_config.endpoint,
    };
    let huobi_provider = HuobiProvider {
        trading_pair: huobi_config.trading_pair,
        endpoint: huobi_config.endpoint,
    };

    // Wrap providers in an Arc and then in new types.
    let kraken_data = KrakenData(Arc::new(kraken_provider));
    let huobi_data = HuobiData(Arc::new(huobi_provider));

    match args[1].as_str() {
        "rest" => run_rest(kraken_data, huobi_data).await,
        "ws" => run_ws().await,
        mode => {
            eprintln!("Unknown mode: {}. Use 'rest' or 'ws'", mode);
            Ok(())
        }
    }
}

/// Run Actix Web server to expose REST endpoints.
async fn run_rest(kraken_data: KrakenData, huobi_data: HuobiData) -> std::io::Result<()> {
    println!("Starting REST server on http://127.0.0.1:8080...");
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(kraken_data.clone()))
            .app_data(actix_web::web::Data::new(huobi_data.clone()))
            .configure(rest::init_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

/// Run the Binance WebSocket integration.
async fn run_ws() -> std::io::Result<()> {
    println!("Starting Binance WebSocket connection...");
    use exchanges::binance_ws::BinanceWebSocket;
    use websocket::websocket_handler::WebSocketHandler;

    // Spawn the Binance WebSocket task.
    tokio::spawn(async {
        let _ = BinanceWebSocket::start().await;
    });

    // Keep the process alive to continue receiving data.
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}

