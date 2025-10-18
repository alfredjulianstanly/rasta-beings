use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::services::ServeFile;

mod handlers;
mod models;
mod state;
mod templates;

use handlers::{admin, cart, shop};
use state::AppState;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
) -> shuttle_axum::ShuttleAxum {
    // Run migrations
    println!("🔄 Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");
    
    println!("✅ Database migrations completed!");
    
    let state = AppState::new(db);

    let logo_bytes = include_bytes!("assets/IMG_2513.jpg");
    
    let router = Router::new()
        .route("/", get(shop::shop_handler))
        .route("/cart", get(cart::view_cart_handler))
        .route("/cart/add", post(cart::add_to_cart_handler))
        .route("/cart/update", post(cart::update_quantity_handler))
        .route("/checkout", post(cart::checkout_handler))
        .route("/admin", get(admin::admin_handler))
        .route("/admin/products", post(admin::add_product_handler))
        .route("/admin/products/delete", post(admin::delete_product_handler))
        .route(
            "/logo.jpg",
            get(|| async {
                (
                    [("content-type", "image/jpeg")],
                    logo_bytes.as_slice(),
                )
            }),
        )
        .with_state(state);

    Ok(router.into())
}
