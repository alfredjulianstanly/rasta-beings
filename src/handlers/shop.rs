use axum::{extract::State, response::Html};
use crate::state::AppState;
use crate::templates::{base_layout, product_card};

pub async fn shop_handler(State(state): State<AppState>) -> Html<String> {
    let products = state.products.lock().unwrap();
    
    let mut product_cards = String::new();
    for product in products.values() {
        product_cards.push_str(&product_card(product));
    }
    
    let content = format!(
        r##"<div style="text-align: center; margin-bottom: 40px; position: relative;">
            <div style="font-size: 2rem; margin-bottom: 15px; opacity: 0.6;">✊🏿</div>
            <div class="rasta-accent" style="width: 200px; margin: 0 auto 20px;"></div>
            <h2 style="color: var(--rasta-gold); font-size: 3rem; margin-bottom: 15px; font-family: 'Philosopher', serif; text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);">
                Our Collection
            </h2>
            <p style="color: var(--light); font-size: 1.1rem; letter-spacing: 1px; margin-bottom: 10px;">
                <span style="color: var(--rasta-red);">One Love</span> 
                <span class="unity-symbol">•</span>
                <span style="color: var(--rasta-gold);">One Heart</span>
                <span class="unity-symbol">•</span>
                <span style="color: var(--rasta-green);">One Destiny</span>
            </p>
            <p style="color: var(--rasta-green); font-size: 0.95rem; opacity: 0.8;">
                🌿 Peace · Unity · Respect for Nature 🌿
            </p>
            <div style="font-size: 1.5rem; margin-top: 10px; opacity: 0.5;">☮️</div>
        </div>
        
        <div class="product-grid">
            {}
        </div>
        
        <div style="text-align: center; margin-top: 50px; padding: 30px; background: rgba(39, 174, 96, 0.05); border-radius: 15px; border: 1px solid rgba(39, 174, 96, 0.2);">
            <div style="font-size: 2rem; margin-bottom: 10px;">🦁</div>
            <p style="color: var(--rasta-green); font-size: 1rem; font-style: italic;">
                "Emancipate yourselves from mental slavery, none but ourselves can free our minds."
            </p>
            <p style="color: var(--accent); font-size: 0.85rem; margin-top: 8px;">- Bob Marley</p>
        </div>"##,
        if product_cards.is_empty() {
            r##"<div style="grid-column: 1 / -1; text-align: center; padding: 60px; color: var(--accent);">
                <div style="font-size: 4rem; margin-bottom: 20px;">🌿</div>
                <h3 style="color: var(--rasta-gold); font-family: 'Philosopher', serif;">No products yet</h3>
                <p>Check back soon for our collection</p>
            </div>"##.to_string()
        } else {
            product_cards
        }
    );
    
    Html(base_layout("Shop", &content))
}
