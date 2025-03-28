// rest/mod.rs
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/kraken/mid_price")]
async fn kraken_mid_price() -> impl Responder {

    match crate::exchanges::kraken_rest_api::fetch_order_book("XBTUSDT").await {
        Ok(order_book) => {
            if let Some((best_bid, best_ask, mid)) = crate::exchanges::kraken_rest_api::compute_mid_price(&order_book) {
                HttpResponse::Ok().json(json!({
                   "Highest Bid": best_bid,
                    "Lowest Ask": best_ask,
                    "mid_price": mid,
                }))
            } else {
                HttpResponse::InternalServerError().body("Failed to compute mid price")
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(kraken_mid_price);
}
