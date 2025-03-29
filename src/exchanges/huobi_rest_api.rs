// src/exchanges/huobi_rest_api.rs
use serde::Deserialize;
use reqwest;
use async_trait::async_trait;
use crate::exchanges::orderbook_provider::{OrderBookData, OrderBookProvider};

/// Huobi’s order book tick data. Huobi returns bids and asks as arrays of numbers:
/// [price, volume]
#[derive(Debug, Deserialize)]
pub struct HuobiTick {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
    pub ts: u64,
    pub version: u64,
}

/// Huobi’s response structure.
#[derive(Debug, Deserialize)]
pub struct HuobiResponse {
    pub status: String,
    pub ch: String,
    pub ts: u64,
    pub tick: HuobiTick,
}

/// Fetch the order book from Huobi's REST API using the given endpoint and trading pair.
/// For Huobi, the API expects the trading pair (symbol) in lowercase and requires a type parameter.
/// We hardcode type=step0 here.
pub async fn fetch_order_book(trading_pair: &str, endpoint: &str) -> Result<HuobiResponse, Box<dyn std::error::Error>> {
    let symbol = trading_pair.to_lowercase();
    let url = format!("{}?symbol={}&type=step0", endpoint, symbol);
    let response = reqwest::get(&url).await?;
    let huobi_response: HuobiResponse = response.json().await?;
    if huobi_response.status != "ok" {
        return Err("Huobi API returned non-ok status".into());
    }
    Ok(huobi_response)
}

/// Compute the best bid, best ask, and mid-price from Huobi's order book data.
pub fn compute_bid_ask_mid(response: &HuobiResponse) -> Option<(f64, f64, f64)> {
    let best_bid = response.tick.bids.first()?.0;  // Price is the first element in the tuple.
    let best_ask = response.tick.asks.first()?.0;
    let mid_price = (best_bid + best_ask) / 2.0;
    Some((best_bid, best_ask, mid_price))
}

/// A Huobi provider that implements the OrderBookProvider trait.
pub struct HuobiProvider {
    pub trading_pair: String,
    pub endpoint: String,
}

#[async_trait]
impl OrderBookProvider for HuobiProvider {
    async fn get_order_book_data(&self) -> Result<OrderBookData, Box<dyn std::error::Error>> {
        let response = fetch_order_book(&self.trading_pair, &self.endpoint).await?;
        if let Some((best_bid, best_ask, mid_price)) = compute_bid_ask_mid(&response) {
            Ok(OrderBookData {
                best_bid,
                best_ask,
                mid_price,
            })
        } else {
            Err("Failed to compute order book data from Huobi".into())
        }
    }
}
