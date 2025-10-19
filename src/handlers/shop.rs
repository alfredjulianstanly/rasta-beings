use axum::{extract::State, response::Html};
use crate::models::Product;
use crate::state::AppState;
use crate::templates::base_layout;

pub async fn shop_handler(State(state): State<AppState>) -> Html<String> {
    let products = sqlx::query_as::<_, Product>(
        "SELECT * FROM products ORDER BY id DESC"
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_else(|e| {
        println!("Error fetching products: {:?}", e);
        vec![]
    });
    
    let mut products_html = String::new();
    
    for product in products {
        // Check if it's an image URL (either R2 or base64)
        let image_html = if product.icon.starts_with("http") {
            // R2 URL - use as image src
            format!(r##"<img src="{}" style="width: 100%; height: 250px; object-fit: cover; border-radius: 12px 12px 0 0;" alt="{}">"##, 
                product.icon, product.name)
        } else if product.icon.starts_with("data:image") {
            // Base64 - use as image src
            format!(r##"<img src="{}" style="width: 100%; height: 250px; object-fit: cover; border-radius: 12px 12px 0 0;" alt="{}">"##, 
                product.icon, product.name)
        } else {
            // Fallback - show emoji or placeholder
            format!(r##"<div style="width: 100%; height: 250px; display: flex; align-items: center; justify-content: center; background: rgba(255, 255, 255, 0.05); border-radius: 12px 12px 0 0; font-size: 4rem;">{}</div>"##,
                product.icon)
        };
        
        products_html.push_str(&format!(
            r##"<div style="background: rgba(255, 255, 255, 0.03); border-radius: 15px; overflow: hidden; border: 1px solid rgba(212, 175, 55, 0.2); transition: all 0.3s ease; box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);" onmouseover="this.style.transform='translateY(-8px)'; this.style.borderColor='rgba(243, 156, 18, 0.5)'; this.style.boxShadow='0 8px 25px rgba(243, 156, 18, 0.3)';" onmouseout="this.style.transform=''; this.style.borderColor='rgba(212, 175, 55, 0.2)'; this.style.boxShadow='0 4px 15px rgba(0, 0, 0, 0.2)';">
                {}
                <div style="padding: 20px;">
                    <div style="display: flex; align-items: center; gap: 10px; margin-bottom: 12px;">
                        <h3 style="color: var(--rasta-gold); font-family: 'Philosopher', serif; margin: 0; font-size: 1.4rem;">{}</h3>
                        <span style="font-size: 1.2rem;">🌿</span>
                    </div>
                    <p style="color: var(--light); margin-bottom: 15px; opacity: 0.9; line-height: 1.6;">{}</p>
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 15px;">
                        <div style="font-size: 1.8rem; font-weight: bold; color: var(--secondary-gold); font-family: 'Philosopher', serif;">₹{}</div>
                        <form method="post" action="/cart/add" style="margin: 0;">
                            <input type="hidden" name="product_id" value="{}">
                            <button type="submit" style="padding: 12px 25px; background: linear-gradient(135deg, var(--rasta-gold) 0%, var(--secondary-gold) 100%); color: var(--primary); border: none; border-radius: 8px; cursor: pointer; font-weight: 700; text-transform: uppercase; letter-spacing: 1px; font-size: 0.9rem; transition: all 0.3s ease; box-shadow: 0 4px 15px rgba(243, 156, 18, 0.3);" onmouseover="this.style.transform='scale(1.05)'; this.style.boxShadow='0 6px 20px rgba(243, 156, 18, 0.5)';" onmouseout="this.style.transform=''; this.style.boxShadow='0 4px 15px rgba(243, 156, 18, 0.3)';">
                                Add to Cart
                            </button>
                        </form>
                    </div>
                </div>
            </div>"##,
            image_html,
            product.name,
            product.description,
            product.price,
            product.id
        ));
    }
    
    let content = format!(
        r##"<div style="text-align: center; margin-bottom: 50px;">
            <h1 style="color: var(--rasta-gold); font-size: 3.5rem; margin-bottom: 15px; font-family: 'Philosopher', serif; text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);">
                Our Collection
            </h1>
            <p style="color: var(--light); font-size: 1.2rem; margin-bottom: 10px;">
                <span style="color: var(--rasta-red);">One Love</span> • 
                <span style="color: var(--rasta-gold);">One Heart</span> • 
                <span style="color: var(--rasta-green);">One Destiny</span>
            </p>
            <p style="color: var(--rasta-green); font-size: 0.95rem; opacity: 0.9;">
                🌿 Peace • Unity • Respect for Nature 🌿
            </p>
            <div style="margin-top: 20px; font-size: 2rem;">☮️</div>
        </div>
        
        <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 30px; margin-top: 30px;">
            {}
        </div>"##,
        products_html
    );
    
    Html(base_layout("Rasta Beings", &content))
}
