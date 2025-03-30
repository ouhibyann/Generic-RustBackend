// src/rest/orderbook_response.rs
use actix_web::HttpResponse;
use serde_json::json;
use std::sync::Arc;
use crate::exchanges::orderbook_provider::OrderBookProvider;

pub async fn order_book_response(provider: Arc<dyn OrderBookProvider + Send + Sync>) -> HttpResponse {
    match provider.get_order_book_data().await {
        Ok(book) => HttpResponse::Ok().json(json!({
            "best_bid": book.best_bid,
            "best_ask": book.best_ask,
            "mid_price": book.mid_price,
        })),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}
