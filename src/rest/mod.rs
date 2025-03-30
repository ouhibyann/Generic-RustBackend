// src/rest/mod.rs
use actix_web::{get, web, Responder};
use std::sync::Arc;
use crate::exchanges::orderbook_provider::OrderBookProvider;
use crate::rest::orderbook_response::order_book_response;

mod orderbook_response;

// Newtypes to distinguish providers.
#[derive(Clone)]
pub struct KrakenData(pub Arc<dyn OrderBookProvider + Send + Sync>);

#[derive(Clone)]
pub struct HuobiData(pub Arc<dyn OrderBookProvider + Send + Sync>);

#[get("/kraken/mid_price")]
async fn kraken_order_book(data: web::Data<KrakenData>) -> impl Responder {
    order_book_response(data.0.clone()).await
}

#[get("/huobi/mid_price")]
async fn huobi_order_book(data: web::Data<HuobiData>) -> impl Responder {
    order_book_response(data.0.clone()).await
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(kraken_order_book);
    cfg.service(huobi_order_book);
}
