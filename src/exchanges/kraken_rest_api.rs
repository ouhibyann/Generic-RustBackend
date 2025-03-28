// exchanges/kraken_rest_api.rs
use serde::Deserialize;
use std::collections::HashMap;
use reqwest; // You may need to add reqwest to Cargo.toml

// The structure below is simplified; adjust the types to match Kraken's API response.
#[derive(Debug, Deserialize)]
pub struct OrderBook {
    pub asks: Vec<(String, String, f64)>, // [price, volume, timestamp]
    pub bids: Vec<(String, String, f64)>,
}

#[derive(Debug, Deserialize)]
struct KrakenResponse {
    pub result: HashMap<String, OrderBook>,
}

/// Fetch the order book for the given trading pair (e.g., "XBTUSDT")
pub async fn fetch_order_book(pair: &str) -> Result<OrderBook, Box<dyn std::error::Error>> {
    let url = format!("https://api.kraken.com/0/public/Depth?pair={}", pair);
    let response = reqwest::get(&url).await?;
    let kraken_response: KrakenResponse = response.json().await?;

    // The key returned by Kraken may not exactly match the request pair,
    // so we grab the first entry.
    let order_book = kraken_response
        .result
        .into_iter()
        .next()
        .map(|(_, ob)| ob)
        .ok_or("No order book data found")?;
    Ok(order_book)
}

/// Compute the mid-price from the best bid and ask in the order book.
pub fn compute_mid_price(order_book: &OrderBook) -> Option<(f64, f64, f64)> {
    // Extract the best bid and ask prices from the tuples.
    let (best_bid_price, _, _) = order_book.bids.first()?;
    let (best_ask_price, _, _) = order_book.asks.first()?;
    let best_bid = best_bid_price.parse::<f64>().ok()?;
    let best_ask = best_ask_price.parse::<f64>().ok()?;
    let mid_price = (best_ask + best_ask) / 2.0;
    Some((best_bid, best_ask, mid_price))
}
