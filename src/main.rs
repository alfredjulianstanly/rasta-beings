mod handlers;
mod models;
mod state;
mod templates;

use axum::{routing::{get, post}, Router, response::Response, body::Body};
use handlers::{
    shop_handler, 
    add_to_cart_handler, 
    view_cart_handler,
    update_quantity_handler,
    checkout_handler,
    admin_handler,
    add_product_handler,
    delete_product_handler
};
use state::AppState;

async fn logo_handler() -> Response<Body> {
    let logo_bytes = include_bytes!("assets/IMG_2513.jpg");
    Response::builder()
        .header("content-type", "image/jpeg")
        .header("cache-control", "public, max-age=31536000")
        .body(Body::from(&logo_bytes[..]))
        .unwrap()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app_state = AppState::new();
    
    let router = Router::new()
        .route("/", get(shop_handler))
        .route("/logo.jpg", get(logo_handler))
        .route("/cart", get(view_cart_handler))
        .route("/cart/add", post(add_to_cart_handler))
        .route("/cart/update", post(update_quantity_handler))
        .route("/checkout", post(checkout_handler))
        .route("/admin", get(admin_handler))
        .route("/admin/products", post(add_product_handler))
        .route("/admin/products/delete", post(delete_product_handler))
        .with_state(app_state);

    Ok(router.into())
}
