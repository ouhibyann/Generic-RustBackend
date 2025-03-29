// src/exchanges/kraken_rest_api.rs
use serde::Deserialize;
use std::collections::HashMap;
use reqwest;
use async_trait::async_trait;
use crate::exchanges::orderbook_provider::{OrderBookData, OrderBookProvider};

#[derive(Debug, Deserialize)]
pub struct OrderBook {
    // Kraken returns each order as an array: [price (string), volume (string), timestamp (number)]
    pub asks: Vec<(String, String, f64)>,
    pub bids: Vec<(String, String, f64)>,
}

#[derive(Debug, Deserialize)]
struct KrakenResponse {
    pub result: HashMap<String, OrderBook>,
}

/// Fetch the order book from Kraken's REST API using the given endpoint and trading pair.
pub async fn fetch_order_book(pair: &str, endpoint: &str) -> Result<OrderBook, Box<dyn std::error::Error>> {
    let url = format!("{}?pair={}", endpoint, pair);
    let response = reqwest::get(&url).await?;
    let kraken_response: KrakenResponse = response.json().await?;
    let order_book = kraken_response
        .result
        .into_iter()
        .next()  // Kraken may return a key that doesn't exactly match the requested pair.
        .map(|(_, ob)| ob)
        .ok_or("No order book data found")?;
    Ok(order_book)
}

/// Compute the best bid, best ask, and mid-price from the order book.
pub fn compute_bid_ask_mid(order_book: &OrderBook) -> Option<(f64, f64, f64)> {
    let (best_bid_price, _, _) = order_book.bids.first()?;
    let (best_ask_price, _, _) = order_book.asks.first()?;
    let best_bid = best_bid_price.parse::<f64>().ok()?;
    let best_ask = best_ask_price.parse::<f64>().ok()?;
    let mid_price = (best_bid + best_ask) / 2.0;
    Some((best_bid, best_ask, mid_price))
}

/// A Kraken provider that implements the OrderBookProvider trait.
pub struct KrakenProvider {
    pub trading_pair: String,
    pub endpoint: String,
}

#[async_trait]
impl OrderBookProvider for KrakenProvider {
    async fn get_order_book_data(&self) -> Result<OrderBookData, Box<dyn std::error::Error>> {
        let order_book = fetch_order_book(&self.trading_pair, &self.endpoint).await?;
        if let Some((best_bid, best_ask, mid_price)) = compute_bid_ask_mid(&order_book) {
            Ok(OrderBookData {
                best_bid,
                best_ask,
                mid_price,
            })
        } else {
            Err("Failed to compute order book data".into())
        }
    }
}
