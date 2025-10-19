use axum::{
    extract::State,
    response::{Html, Redirect},
    Form,
};
use serde::Deserialize;
use sqlx::Row;
use rust_decimal::Decimal;
use crate::state::AppState;
use crate::templates::base_layout;

#[derive(Deserialize)]
pub struct AddToCartForm {
    product_id: i32,
}

pub async fn add_to_cart_handler(
    State(state): State<AppState>,
    Form(form): Form<AddToCartForm>,
) -> Redirect {
    let session_id = "default_user";
    
    let cart_result = sqlx::query(
        "INSERT INTO carts (session_id) VALUES ($1)
         ON CONFLICT (session_id) DO UPDATE SET updated_at = CURRENT_TIMESTAMP
         RETURNING id"
    )
    .bind(session_id)
    .fetch_one(&state.db)
    .await;
    
    if let Ok(cart) = cart_result {
        let cart_id: i32 = cart.get("id");
        
        let _ = sqlx::query(
            "INSERT INTO cart_items (cart_id, product_id, quantity)
             VALUES ($1, $2, 1)
             ON CONFLICT (cart_id, product_id) 
             DO UPDATE SET quantity = cart_items.quantity + 1"
        )
        .bind(cart_id)
        .bind(form.product_id)
        .execute(&state.db)
        .await;
    }
    
    Redirect::to("/cart")
}

#[derive(Deserialize)]
pub struct UpdateQuantityForm {
    product_id: i32,
    action: String,
}

pub async fn update_quantity_handler(
    State(state): State<AppState>,
    Form(form): Form<UpdateQuantityForm>,
) -> Redirect {
    let session_id = "default_user";
    
    if let Ok(cart) = sqlx::query("SELECT id FROM carts WHERE session_id = $1")
        .bind(session_id)
        .fetch_one(&state.db)
        .await
    {
        let cart_id: i32 = cart.get("id");
        
        if form.action == "increase" {
            let _ = sqlx::query(
                "UPDATE cart_items SET quantity = quantity + 1
                 WHERE cart_id = $1 AND product_id = $2"
            )
            .bind(cart_id)
            .bind(form.product_id)
            .execute(&state.db)
            .await;
        } else if form.action == "decrease" {
            if let Ok(item) = sqlx::query(
                "SELECT quantity FROM cart_items WHERE cart_id = $1 AND product_id = $2"
            )
            .bind(cart_id)
            .bind(form.product_id)
            .fetch_one(&state.db)
            .await
            {
                let quantity: i32 = item.get("quantity");
                
                if quantity > 1 {
                    let _ = sqlx::query(
                        "UPDATE cart_items SET quantity = quantity - 1
                         WHERE cart_id = $1 AND product_id = $2"
                    )
                    .bind(cart_id)
                    .bind(form.product_id)
                    .execute(&state.db)
                    .await;
                } else {
                    let _ = sqlx::query(
                        "DELETE FROM cart_items WHERE cart_id = $1 AND product_id = $2"
                    )
                    .bind(cart_id)
                    .bind(form.product_id)
                    .execute(&state.db)
                    .await;
                }
            }
        }
    }
    
    Redirect::to("/cart")
}

pub async fn view_cart_handler(State(state): State<AppState>) -> Html<String> {
    let session_id = "default_user";
    
    let content = if let Ok(cart) = sqlx::query("SELECT id FROM carts WHERE session_id = $1")
        .bind(session_id)
        .fetch_one(&state.db)
        .await
    {
        let cart_id: i32 = cart.get("id");
        
        let items = sqlx::query(
            "SELECT ci.product_id, ci.quantity, p.name, p.description, p.price, p.icon
             FROM cart_items ci
             JOIN products p ON ci.product_id = p.id
             WHERE ci.cart_id = $1"
        )
        .bind(cart_id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();
        
        if items.is_empty() {
            empty_cart_html()
        } else {
            render_cart_items(&items)
        }
    } else {
        empty_cart_html()
    };
    
    Html(base_layout("Cart", &content))
}

pub async fn checkout_handler(State(state): State<AppState>) -> Html<String> {
    let session_id = "default_user";
    
    let total = if let Ok(cart) = sqlx::query("SELECT id FROM carts WHERE session_id = $1")
        .bind(session_id)
        .fetch_one(&state.db)
        .await
    {
        let cart_id: i32 = cart.get("id");
        
        let items = sqlx::query(
            "SELECT ci.quantity, p.price
             FROM cart_items ci
             JOIN products p ON ci.product_id = p.id
             WHERE ci.cart_id = $1"
        )
        .bind(cart_id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();
        
        let total: Decimal = items.iter().map(|i| {
            let quantity: i32 = i.get("quantity");
            let price: Decimal = i.get("price");
            price * Decimal::from(quantity)
        }).sum();
        
        let _ = sqlx::query("DELETE FROM cart_items WHERE cart_id = $1")
            .bind(cart_id)
            .execute(&state.db)
            .await;
        
        total
    } else {
        Decimal::ZERO
    };
    
    let content = format!(
        r##"<div style="text-align: center; padding: 60px 20px; max-width: 600px; margin: 0 auto;">
            <div style="position: relative; width: 120px; height: 120px; margin: 0 auto 30px;">
                <div style="background: linear-gradient(135deg, var(--rasta-red), var(--rasta-gold), var(--rasta-green)); border-radius: 50%; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
                    <div style="font-size: 3rem;">✊🏿</div>
                </div>
                <div style="position: absolute; top: -10px; right: -10px; font-size: 1.5rem;">🌿</div>
                <div style="position: absolute; bottom: -10px; left: -10px; font-size: 1.5rem;">☮️</div>
            </div>
            
            <h2 style="color: var(--rasta-gold); font-size: 3rem; margin-bottom: 20px; font-family: 'Philosopher', serif;">
                Order Complete!
            </h2>
            
            <p style="color: var(--light); font-size: 1.2rem; line-height: 1.8; margin-bottom: 15px;">
                Thank you for your purchase from <strong style="color: var(--rasta-gold);">Rasta Beings</strong>
            </p>
            
            <div style="background: rgba(255, 255, 255, 0.05); padding: 30px; border-radius: 15px; border: 1px solid rgba(243, 156, 18, 0.3); margin: 30px 0;">
                <p style="color: var(--rasta-green); margin-bottom: 10px; text-transform: uppercase; letter-spacing: 2px; font-size: 0.9rem;">Order Total</p>
                <div style="font-size: 2.5rem; color: var(--rasta-gold); font-weight: bold; font-family: 'Philosopher', serif;">
                    ₹{}
                </div>
            </div>
            
            <a href="/" style="display: inline-block; padding: 15px 40px; background: linear-gradient(135deg, var(--rasta-gold) 0%, var(--secondary-gold) 100%); color: var(--primary); text-decoration: none; border-radius: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: 1px;">
                Continue Shopping
            </a>
            
            <div style="margin-top: 40px; padding-top: 30px; border-top: 1px solid rgba(243, 156, 18, 0.2);">
                <p style="color: var(--rasta-green); font-size: 1rem; font-style: italic;">
                    "One Love, One Heart"
                </p>
                <p style="color: var(--accent); font-size: 0.85rem;">🌿 Blessings 🌿</p>
            </div>
        </div>"##,
        total
    );
    
    Html(base_layout("Order Complete", &content))
}

fn empty_cart_html() -> String {
    r##"<div style="text-align: center; padding: 60px;">
        <div style="font-size: 4rem; margin-bottom: 20px;">🛒</div>
        <h2 style="color: var(--rasta-gold); font-family: 'Philosopher', serif;">Your cart is empty</h2>
        <p style="margin: 15px 0;">Start shopping to add items</p>
        <a href="/" style="display: inline-block; margin-top: 20px; padding: 14px 30px; background: linear-gradient(135deg, var(--rasta-gold) 0%, var(--secondary-gold) 100%); color: var(--primary); text-decoration: none; border-radius: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: 1px;">
            Browse Products
        </a>
    </div>"##.to_string()
}

fn render_cart_items(items: &[sqlx::postgres::PgRow]) -> String {
    use sqlx::Row;
    
    let mut cart_html = String::from(r##"<div style="text-align: center; margin-bottom: 30px;">
        <h2 style="color: var(--rasta-gold); font-size: 3rem; font-family: 'Philosopher', serif;">Your Cart</h2>
    </div>"##);
    
    let mut total = Decimal::ZERO;
    
    for item in items {
        let product_id: i32 = item.get("product_id");
        let quantity: i32 = item.get("quantity");
        let name: String = item.get("name");
        let price: Decimal = item.get("price");
        let icon: String = item.get("icon");
        let subtotal = price * Decimal::from(quantity);
        total += subtotal;
        
        // Render image properly based on URL type
        let thumbnail = if icon.starts_with("http") {
            format!(r##"<img src="{}" style="width: 80px; height: 80px; object-fit: cover; border-radius: 50%; border: 2px solid var(--rasta-gold);" alt="{}">"##, icon, name)
        } else if icon.starts_with("data:image") {
            format!(r##"<img src="{}" style="width: 80px; height: 80px; object-fit: cover; border-radius: 50%; border: 2px solid var(--rasta-gold);" alt="{}">"##, icon, name)
        } else {
            format!(r##"<div style="font-size: 3rem;">{}</div>"##, icon)
        };
        
        cart_html.push_str(&format!(
            r##"<div style="background: rgba(255, 255, 255, 0.03); padding: 20px; border-radius: 12px; margin-bottom: 15px; border: 1px solid rgba(243, 156, 18, 0.2);">
                <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 15px; flex-wrap: wrap; gap: 15px;">
                    <div style="display: flex; align-items: center; gap: 15px;">
                        {}
                        <div>
                            <h3 style="color: var(--rasta-gold); font-family: 'Philosopher', serif;">{}</h3>
                            <p style="color: var(--accent); font-size: 0.9rem;">₹{} each</p>
                        </div>
                    </div>
                    <div>
                        <p style="font-size: 1.4rem; font-weight: bold; color: var(--rasta-gold);">₹{}</p>
                    </div>
                </div>
                <div style="display: flex; align-items: center; gap: 15px; justify-content: center;">
                    <form method="post" action="/cart/update" style="display: inline;">
                        <input type="hidden" name="product_id" value="{}">
                        <input type="hidden" name="action" value="decrease">
                        <button type="submit" style="width: 40px; height: 40px; background: rgba(231, 76, 60, 0.2); color: var(--rasta-red); border: 2px solid var(--rasta-red); border-radius: 50%; font-size: 1.3rem; cursor: pointer;">−</button>
                    </form>
                    <span style="min-width: 50px; text-align: center; font-weight: bold; font-size: 1.2rem;">{}</span>
                    <form method="post" action="/cart/update" style="display: inline;">
                        <input type="hidden" name="product_id" value="{}">
                        <input type="hidden" name="action" value="increase">
                        <button type="submit" style="width: 40px; height: 40px; background: var(--rasta-green); color: white; border: none; border-radius: 50%; font-size: 1.3rem; cursor: pointer;">+</button>
                    </form>
                </div>
            </div>"##,
            thumbnail, name, price, subtotal, product_id, quantity, product_id
        ));
    }
    
    cart_html.push_str(&format!(
        r##"<div style="margin-top: 30px; padding: 30px; background: rgba(255, 255, 255, 0.03); border-radius: 15px; border: 1px solid rgba(243, 156, 18, 0.3);">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 25px;">
                <h3 style="color: var(--light); font-family: 'Philosopher', serif; font-size: 1.5rem;">Total</h3>
                <div style="font-size: 2.2rem; color: var(--rasta-gold); font-weight: bold; font-family: 'Philosopher', serif;">₹{}</div>
            </div>
            <div style="display: flex; gap: 15px; flex-wrap: wrap;">
                <a href="/" style="flex: 1; min-width: 200px; padding: 15px; background: transparent; border: 2px solid var(--rasta-gold); color: var(--rasta-gold); text-decoration: none; border-radius: 10px; font-weight: 700; text-align: center; text-transform: uppercase;">
                    Continue Shopping
                </a>
                <form method="post" action="/checkout" style="flex: 1; min-width: 200px;">
                    <button type="submit" style="width: 100%; padding: 15px; background: linear-gradient(135deg, var(--rasta-gold) 0%, var(--secondary-gold) 100%); color: var(--primary); border: none; border-radius: 10px; font-weight: 700; cursor: pointer; text-transform: uppercase;">
                        Checkout
                    </button>
                </form>
            </div>
        </div>"##,
        total
    ));
    
    cart_html
}
