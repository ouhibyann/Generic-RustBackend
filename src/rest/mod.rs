// src/rest/mod.rs
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use std::sync::Arc;
use crate::exchanges::orderbook_provider::OrderBookProvider;

// Newtypes to distinguish providers.
#[derive(Clone)]
pub struct KrakenData(pub Arc<dyn OrderBookProvider + Send + Sync>);

#[derive(Clone)]
pub struct HuobiData(pub Arc<dyn OrderBookProvider + Send + Sync>);

#[get("/kraken/mid_price")]
async fn kraken_order_book(data: web::Data<KrakenData>) -> impl Responder {
    match data.0.get_order_book_data().await {
        Ok(book) => HttpResponse::Ok().json(json!({
            "best_bid": book.best_bid,
            "best_ask": book.best_ask,
            "mid_price": book.mid_price,
        })),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[get("/huobi/mid_price")]
async fn huobi_order_book(data: web::Data<HuobiData>) -> impl Responder {
    match data.0.get_order_book_data().await {
        Ok(book) => HttpResponse::Ok().json(json!({
            "best_bid": book.best_bid,
            "best_ask": book.best_ask,
            "mid_price": book.mid_price,
        })),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(kraken_order_book);
    cfg.service(huobi_order_book);
}
