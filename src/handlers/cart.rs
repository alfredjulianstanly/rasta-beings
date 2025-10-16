use axum::{
    extract::State,
    response::{Html, Redirect},
    Form,
};
use serde::Deserialize;
use crate::models::Cart;
use crate::state::AppState;
use crate::templates::base_layout;

#[derive(Deserialize)]
pub struct AddToCartForm {
    product_id: u64,
}

#[derive(Deserialize)]
pub struct UpdateQuantityForm {
    product_id: u64,
    action: String,
}

pub async fn add_to_cart_handler(
    State(state): State<AppState>,
    Form(form): Form<AddToCartForm>,
) -> Redirect {
    let session_id = "default_user".to_string();
    
    let mut carts = state.carts.lock().unwrap();
    let cart = carts.entry(session_id).or_insert_with(Cart::new);
    cart.add_item(form.product_id);
    
    Redirect::to("/cart")
}

pub async fn update_quantity_handler(
    State(state): State<AppState>,
    Form(form): Form<UpdateQuantityForm>,
) -> Redirect {
    let session_id = "default_user".to_string();
    
    let mut carts = state.carts.lock().unwrap();
    if let Some(cart) = carts.get_mut(&session_id) {
        if let Some(item) = cart.items.iter_mut().find(|i| i.product_id == form.product_id) {
            if form.action == "increase" {
                item.quantity += 1;
            } else if form.action == "decrease" {
                if item.quantity > 1 {
                    item.quantity -= 1;
                } else {
                    cart.items.retain(|i| i.product_id != form.product_id);
                }
            }
        }
    }
    
    Redirect::to("/cart")
}

pub async fn view_cart_handler(State(state): State<AppState>) -> Html<String> {
    let session_id = "default_user".to_string();
    
    let carts = state.carts.lock().unwrap();
    let cart = carts.get(&session_id);
    
    let content = if let Some(cart) = cart {
        if cart.items.is_empty() {
            empty_cart_html()
        } else {
            render_cart(cart, &state)
        }
    } else {
        empty_cart_html()
    };
    
    Html(base_layout("Cart", &content))
}

pub async fn checkout_handler(State(state): State<AppState>) -> Html<String> {
    let session_id = "default_user".to_string();
    
    let carts = state.carts.lock().unwrap();
    let cart = carts.get(&session_id);
    
    let total = if let Some(cart) = cart {
        let products = state.products.lock().unwrap();
        cart.items.iter()
            .filter_map(|item| products.get(&item.product_id).map(|p| p.price * item.quantity as f64))
            .sum::<f64>()
    } else {
        0.0
    };
    
    drop(carts);
    let mut carts = state.carts.lock().unwrap();
    carts.remove(&session_id);
    
    let content = format!(
        r##"<div style="text-align: center; padding: 60px 20px; max-width: 600px; margin: 0 auto;">
            <div style="position: relative; width: 120px; height: 120px; margin: 0 auto 30px;">
                <div style="background: linear-gradient(135deg, var(--rasta-red), var(--rasta-gold), var(--rasta-green)); border-radius: 50%; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; animation: pulse 2s ease-in-out infinite;">
                    <div style="font-size: 3rem;">✊🏿</div>
                </div>
                <div style="position: absolute; top: -10px; right: -10px; font-size: 1.5rem;">🌿</div>
                <div style="position: absolute; bottom: -10px; left: -10px; font-size: 1.5rem;">☮️</div>
            </div>
            <style>
                @keyframes pulse {{
                    0%, 100% {{ transform: scale(1); }}
                    50% {{ transform: scale(1.05); }}
                }}
            </style>
            
            <h2 style="color: var(--rasta-gold); font-size: 3rem; margin-bottom: 20px; font-family: 'Philosopher', serif;">
                Order Complete!
            </h2>
            
            <p style="color: var(--light); font-size: 1.2rem; line-height: 1.8; margin-bottom: 15px;">
                Thank you for your purchase from <strong style="color: var(--rasta-gold);">Rasta Beings</strong>
            </p>
            
            <div style="background: rgba(255, 255, 255, 0.05); padding: 30px; border-radius: 15px; border: 1px solid rgba(243, 156, 18, 0.3); margin: 30px 0; position: relative;">
                <div style="position: absolute; top: 15px; right: 15px; font-size: 1.5rem; opacity: 0.3;">🦁</div>
                <div class="rasta-accent" style="margin-bottom: 20px;"></div>
                <p style="color: var(--rasta-green); margin-bottom: 10px; text-transform: uppercase; letter-spacing: 2px; font-size: 0.9rem;">Order Total</p>
                <div style="font-size: 2.5rem; color: var(--rasta-gold); font-weight: bold; font-family: 'Philosopher', serif;">
                    ₹{:.2}
                </div>
            </div>
            
            <p style="color: var(--accent); margin-bottom: 30px; font-size: 1rem;">
                A confirmation email has been sent to your inbox.
            </p>
            
            <a href="/" style="display: inline-block; padding: 15px 40px; background: linear-gradient(135deg, var(--rasta-gold) 0%, var(--secondary-gold) 100%); color: var(--primary); text-decoration: none; border-radius: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: 1px; transition: all 0.3s ease;">
                Continue Shopping
            </a>
            
            <div style="margin-top: 40px; padding-top: 30px; border-top: 1px solid rgba(243, 156, 18, 0.2);">
                <p style="color: var(--rasta-green); font-size: 1rem; font-style: italic; margin-bottom: 10px;">
                    "One Love, One Heart, Let's get together and feel alright"
                </p>
                <p style="color: var(--accent); font-size: 0.85rem;">🌿 Blessings and Respect 🌿</p>
            </div>
        </div>"##,
        total
    );
    
    Html(base_layout("Order Complete", &content))
}

fn empty_cart_html() -> String {
    r##"<div style="text-align: center; padding: 60px; color: var(--accent);">
        <div style="font-size: 4rem; margin-bottom: 20px;">🛒</div>
        <h2 style="color: var(--rasta-gold); font-family: 'Philosopher', serif;">Your cart is empty</h2>
        <p style="margin: 15px 0;">Start shopping to add items</p>
        <a href="/" style="display: inline-block; margin-top: 20px; padding: 14px 30px; background: linear-gradient(135deg, var(--rasta-gold) 0%, var(--secondary-gold) 100%); color: var(--primary); text-decoration: none; border-radius: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: 1px;">
            Browse Products
        </a>
    </div>"##.to_string()
}

fn render_cart(cart: &Cart, state: &AppState) -> String {
    let products = state.products.lock().unwrap();
    let mut cart_html = String::from(r##"<div style="text-align: center; margin-bottom: 30px;">
        <div style="font-size: 1.5rem; margin-bottom: 10px; opacity: 0.6;">☮️</div>
        <div class="rasta-accent" style="width: 200px; margin: 0 auto 20px;"></div>
        <h2 style="color: var(--rasta-gold); font-size: 3rem; font-family: 'Philosopher', serif;">Your Cart</h2>
        <p style="color: var(--rasta-green); font-size: 0.9rem; margin-top: 10px;">🌿 One Love, One Cart 🌿</p>
    </div>"##);
    
    let mut total = 0.0;
    
    for item in &cart.items {
        if let Some(product) = products.get(&item.product_id) {
            let subtotal = product.price * item.quantity as f64;
            total += subtotal;
            
            let thumbnail = if product.icon.starts_with("data:image") {
                format!(r##"<img src="{}" style="width: 80px; height: 80px; object-fit: cover; border-radius: 50%; border: 2px solid var(--rasta-gold);" alt="{}">"##, product.icon, product.name)
            } else {
                format!(r##"<div style="font-size: 3rem;">{}</div>"##, product.icon)
            };
            
            cart_html.push_str(&format!(
                r##"<div style="background: rgba(255, 255, 255, 0.03); padding: 20px; border-radius: 12px; margin-bottom: 15px; border: 1px solid rgba(243, 156, 18, 0.2); position: relative;">
                    <div style="position: absolute; top: 10px; right: 10px; font-size: 1rem; opacity: 0.2;">🌿</div>
                    <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 15px; flex-wrap: wrap; gap: 15px;">
                        <div style="display: flex; align-items: center; gap: 15px; flex: 1; min-width: 200px;">
                            {}
                            <div style="text-align: left;">
                                <h3 style="color: var(--rasta-gold); margin-bottom: 5px; font-family: 'Philosopher', serif;">{}</h3>
                                <p style="color: var(--accent); font-size: 0.9rem;">₹{:.2} each</p>
                            </div>
                        </div>
                        <div style="text-align: right;">
                            <p style="font-size: 1.4rem; font-weight: bold; color: var(--rasta-gold);">₹{:.2}</p>
                        </div>
                    </div>
                    <div style="display: flex; align-items: center; gap: 15px; justify-content: center;">
                        <form method="post" action="/cart/update" style="display: inline;">
                            <input type="hidden" name="product_id" value="{}">
                            <input type="hidden" name="action" value="decrease">
                            <button type="submit" style="width: 40px; height: 40px; background: rgba(231, 76, 60, 0.2); color: var(--rasta-red); border: 2px solid var(--rasta-red); border-radius: 50%; font-size: 1.3rem; font-weight: bold; cursor: pointer; transition: all 0.3s ease;">−</button>
                        </form>
                        <span style="min-width: 50px; text-align: center; font-weight: bold; font-size: 1.2rem; color: var(--light);">{}</span>
                        <form method="post" action="/cart/update" style="display: inline;">
                            <input type="hidden" name="product_id" value="{}">
                            <input type="hidden" name="action" value="increase">
                            <button type="submit" style="width: 40px; height: 40px; background: var(--rasta-green); color: white; border: none; border-radius: 50%; font-size: 1.3rem; font-weight: bold; cursor: pointer; transition: all 0.3s ease;">+</button>
                        </form>
                    </div>
                </div>"##,
                thumbnail, product.name, product.price, subtotal, product.id, item.quantity, product.id
            ));
        }
    }
    
    cart_html.push_str(&format!(
        r##"<div class="rasta-accent" style="margin: 30px 0;"></div>
        <div style="margin-top: 30px; padding: 30px; background: rgba(255, 255, 255, 0.03); border-radius: 15px; border: 1px solid rgba(243, 156, 18, 0.3); position: relative;">
            <div style="position: absolute; top: 15px; right: 15px; font-size: 1.5rem; opacity: 0.2;">✊🏿</div>
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 25px; flex-wrap: wrap; gap: 15px;">
                <h3 style="color: var(--light); font-family: 'Philosopher', serif; font-size: 1.5rem;">Total</h3>
                <div style="font-size: 2.2rem; color: var(--rasta-gold); font-weight: bold; font-family: 'Philosopher', serif;">₹{:.2}</div>
            </div>
            <div style="display: flex; gap: 15px; flex-wrap: wrap;">
                <a href="/" style="flex: 1; min-width: 200px; padding: 15px; background: transparent; border: 2px solid var(--rasta-gold); color: var(--rasta-gold); text-decoration: none; border-radius: 10px; font-weight: 700; text-align: center; text-transform: uppercase; letter-spacing: 1px; transition: all 0.3s ease;">
                    Continue Shopping
                </a>
                <form method="post" action="/checkout" style="flex: 1; min-width: 200px;">
                    <button type="submit" style="width: 100%; padding: 15px; background: linear-gradient(135deg, var(--rasta-gold) 0%, var(--secondary-gold) 100%); color: var(--primary); border: none; border-radius: 10px; font-weight: 700; cursor: pointer; text-transform: uppercase; letter-spacing: 1px; transition: all 0.3s ease;">
                        Checkout
                    </button>
                </form>
            </div>
        </div>"##,
        total
    ));
    
    cart_html
}
