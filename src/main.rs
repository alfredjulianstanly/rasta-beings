use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

mod handlers;
mod models;
mod state;
mod templates;
mod r2;

use handlers::{
    admin::{add_product_handler, admin_handler, delete_product_handler},
    cart::{add_to_cart_handler, checkout_handler, update_quantity_handler, view_cart_handler},
    shop::shop_handler,
};
use state::AppState;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // Load R2 secrets into environment
    if let Some(key) = secrets.get("R2_ACCESS_KEY_ID") {
        std::env::set_var("R2_ACCESS_KEY_ID", key);
    }
    if let Some(key) = secrets.get("R2_SECRET_ACCESS_KEY") {
        std::env::set_var("R2_SECRET_ACCESS_KEY", key);
    }
    if let Some(endpoint) = secrets.get("R2_ENDPOINT") {
        std::env::set_var("R2_ENDPOINT", endpoint);
    }
    if let Some(bucket) = secrets.get("R2_BUCKET_NAME") {
        std::env::set_var("R2_BUCKET_NAME", bucket);
    }
    if let Some(url) = secrets.get("R2_PUBLIC_URL") {
        std::env::set_var("R2_PUBLIC_URL", url);
    }
    
    println!("🔄 Running database migrations...");
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run migrations");
    println!("✅ Database migrations completed!");

    let state = AppState { db };

    let router = Router::new()
        .route("/", get(shop_handler))
        .route("/admin", get(admin_handler))
        .route("/admin/products", post(add_product_handler))
        .route("/admin/products/delete", post(delete_product_handler))
        .route("/cart", get(view_cart_handler))
        .route("/cart/add", post(add_to_cart_handler))
        .route("/cart/update", post(update_quantity_handler))
        .route("/checkout", post(checkout_handler))
        .with_state(state);

    Ok(router.into())
}
