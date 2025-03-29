// src/exchanges/orderbook_provider.rs
use async_trait::async_trait;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OrderBookData {
    pub best_bid: f64,
    pub best_ask: f64,
    pub mid_price: f64,
}

#[async_trait]
pub trait OrderBookProvider {
    /// Retrieves best_bid, best_ask, and mid_price for a trading pair.
    async fn get_order_book_data(&self) -> Result<OrderBookData, Box<dyn std::error::Error>>;
}
