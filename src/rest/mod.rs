// src/rest/mod.rs
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use crate::exchanges::orderbook_provider::OrderBookProvider;

#[get("/kraken/mid_price")]
async fn kraken_order_book(provider: web::Data<std::sync::Arc<dyn OrderBookProvider + Send + Sync>>) -> impl Responder {
    match provider.get_order_book_data().await {
        Ok(data) => HttpResponse::Ok().json(json!({
            "best_bid": data.best_bid,
            "best_ask": data.best_ask,
            "mid_price": data.mid_price,
        })),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(kraken_order_book);
}
