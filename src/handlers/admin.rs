use axum::{
    extract::{Query, State},
    response::{Html, Redirect},
    Form,
};
use axum_extra::extract::Multipart;
use serde::Deserialize;
use sqlx::Row;
use rust_decimal::Decimal;
use crate::models::Product;
use crate::state::AppState;
use crate::templates::base_layout;
use crate::r2::R2Client;

#[derive(Deserialize)]
pub struct AdminQuery {
    success: Option<String>,
}

pub async fn admin_handler(State(state): State<AppState>, Query(query): Query<AdminQuery>) -> Html<String> {
    let products = sqlx::query_as::<_, Product>(
        "SELECT * FROM products ORDER BY id DESC"
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_else(|e| {
        println!("Error fetching products: {:?}", e);
        vec![]
    });
    
    let success_modal = if query.success.is_some() {
        r##"<div id="success-toast" style="position: fixed; top: 20px; right: 20px; background: linear-gradient(135deg, #27ae60, #2ecc71); color: white; padding: 20px 30px; border-radius: 12px; box-shadow: 0 8px 30px rgba(39, 174, 96, 0.4); z-index: 1000; animation: slideIn 0.5s ease; border-left: 5px solid #27ae60;">
            <div style="display: flex; align-items: center; gap: 15px;">
                <div style="font-size: 2rem;">✓</div>
                <div>
                    <strong style="display: block; font-size: 1.1rem; margin-bottom: 5px;">Product Added!</strong>
                    <span style="font-size: 0.9rem; opacity: 0.9;">Uploaded to R2 ☁️✨</span>
                </div>
            </div>
        </div>
        <script>
            setTimeout(() => {
                const toast = document.getElementById('success-toast');
                if (toast) {
                    toast.style.animation = 'slideOut 0.5s ease';
                    setTimeout(() => toast.remove(), 500);
                }
            }, 3000);
        </script>
        <style>
            @keyframes slideIn {
                from { transform: translateX(400px); opacity: 0; }
                to { transform: translateX(0); opacity: 1; }
            }
            @keyframes slideOut {
                from { transform: translateX(0); opacity: 1; }
                to { transform: translateX(400px); opacity: 0; }
            }
        </style>"##
    } else {
        ""
    };
    
    let mut products_list = String::new();
    for product in &products {
        let display_icon = if product.icon.starts_with("http") {
            format!(r##"<img src="{}" style="width: 70px; height: 70px; object-fit: cover; border-radius: 50%; margin-right: 15px; border: 2px solid var(--secondary-gold);" alt="{}">"##, product.icon, product.name)
        } else {
            format!(r##"<span style="font-size: 2.5rem; margin-right: 15px;">🖼️</span>"##)
        };
        
        products_list.push_str(&format!(
            r##"<div style="background: rgba(255, 255, 255, 0.03); padding: 20px; border-radius: 12px; margin-bottom: 15px; display: flex; justify-content: space-between; align-items: center; border: 1px solid rgba(212, 175, 55, 0.2);">
                <div style="display: flex; align-items: center; flex: 1;">
                    {}
                    <div>
                        <h4 style="color: var(--secondary-gold); margin-bottom: 8px; font-family: 'Philosopher', serif;">{}</h4>
                        <p style="color: var(--light); opacity: 0.8; font-size: 0.9rem;">{}</p>
                        <p style="font-weight: bold; color: var(--rasta-gold); margin-top: 8px; font-size: 1.1rem;">₹{}</p>
                    </div>
                </div>
                <form method="post" action="/admin/products/delete">
                    <input type="hidden" name="product_id" value="{}">
                    <button type="submit" style="padding: 10px 20px; background: linear-gradient(135deg, #e74c3c, #c0392b); color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600;">Delete</button>
                </form>
            </div>"##,
            display_icon, product.name, product.description, product.price, product.id
        ));
    }
    
    let content = format!(
        r##"{}
        <div class="rasta-accent" style="margin-bottom: 30px;"></div>
        
        <h2 style="color: var(--secondary-gold); font-size: 2.5rem; margin-bottom: 30px; font-family: 'Philosopher', serif; text-align: center;">
            Admin Panel
        </h2>
        
        <div style="background: rgba(255, 255, 255, 0.03); padding: 35px; border-radius: 15px; margin-bottom: 40px; border: 1px solid rgba(212, 175, 55, 0.3);">
            <h3 style="color: var(--secondary-gold); margin-bottom: 25px; font-family: 'Philosopher', serif; font-size: 1.5rem;">Add New Product</h3>
            <form id="product-form" method="post" action="/admin/products" enctype="multipart/form-data">
                <div style="margin-bottom: 20px;">
                    <label style="display: block; margin-bottom: 10px; color: var(--rasta-gold); font-weight: 600;">Product Name</label>
                    <input type="text" name="name" required style="width: 100%; padding: 14px; border: 2px solid rgba(243, 156, 18, 0.3); border-radius: 10px; background: rgba(255, 255, 255, 0.05); color: var(--light); font-size: 1rem;">
                </div>
                
                <div style="margin-bottom: 20px;">
                    <label style="display: block; margin-bottom: 10px; color: var(--rasta-gold); font-weight: 600;">Description</label>
                    <textarea name="description" required style="width: 100%; padding: 14px; border: 2px solid rgba(243, 156, 18, 0.3); border-radius: 10px; background: rgba(255, 255, 255, 0.05); color: var(--light); font-size: 1rem; min-height: 120px; font-family: 'Raleway', sans-serif;"></textarea>
                </div>
                
                <div style="margin-bottom: 20px;">
                    <label style="display: block; margin-bottom: 10px; color: var(--rasta-gold); font-weight: 600;">Price (₹)</label>
                    <input type="number" name="price" step="0.01" required style="width: 100%; padding: 14px; border: 2px solid rgba(243, 156, 18, 0.3); border-radius: 10px; background: rgba(255, 255, 255, 0.05); color: var(--light); font-size: 1rem;">
                </div>
                
                <div style="margin-bottom: 25px;">
                    <label style="display: block; margin-bottom: 10px; color: var(--rasta-gold); font-weight: 600;">Product Image</label>
                    <input type="file" name="image" accept="image/*" required style="width: 100%; padding: 14px; border: 2px solid rgba(243, 156, 18, 0.3); border-radius: 10px; background: rgba(255, 255, 255, 0.05); color: var(--light);">
                </div>
                
                <button type="submit" style="width: 100%; padding: 16px; background: linear-gradient(135deg, var(--secondary-gold) 0%, var(--rasta-gold) 100%); color: var(--primary); border: none; border-radius: 10px; font-weight: 700; cursor: pointer; text-transform: uppercase;">
                    Upload to R2 & Add Product ☁️
                </button>
            </form>
        </div>
        
        <h3 style="color: var(--secondary-gold); margin-bottom: 25px; font-family: 'Philosopher', serif; font-size: 1.8rem; text-align: center;">
            Current Products ({})
        </h3>
        <div>{}</div>"##,
        success_modal,
        products.len(),
        products_list
    );
    
    Html(base_layout("Admin", &content))
}

pub async fn add_product_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Redirect {
    let mut name = String::new();
    let mut description = String::new();
    let mut price_f64 = 0.0;
    let mut image_data: Option<Vec<u8>> = None;
    
    println!("📦 Starting multipart processing...");
    
    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = match field.name() {
            Some(n) => n.to_string(),
            None => continue,
        };
        
        match field_name.as_str() {
            "name" => {
                if let Ok(text) = field.text().await {
                    name = text;
                    println!("✅ Got name: {}", name);
                }
            }
            "description" => {
                if let Ok(text) = field.text().await {
                    description = text;
                    println!("✅ Got description");
                }
            }
            "price" => {
                if let Ok(text) = field.text().await {
                    price_f64 = text.parse().unwrap_or(0.0);
                    println!("✅ Got price: {}", price_f64);
                }
            }
            "image" => {
                if let Ok(data) = field.bytes().await {
                    println!("✅ Got image: {} bytes", data.len());
                    image_data = Some(data.to_vec());
                }
            }
            _ => {}
        }
    }
    
    if !name.is_empty() && !description.is_empty() && image_data.is_some() && price_f64 > 0.0 {
        println!("☁️ Uploading to R2...");
        
        match R2Client::new().await {
            Ok(r2) => {
                match r2.upload_image(image_data.unwrap(), "image/jpeg").await {
                    Ok(image_url) => {
                        println!("✅ Uploaded to R2: {}", image_url);
                        
                        let price = Decimal::from_f64_retain(price_f64).unwrap_or_default();
                        
                        let result = sqlx::query(
                            "INSERT INTO products (name, description, price, icon) VALUES ($1, $2, $3, $4)"
                        )
                        .bind(&name)
                        .bind(&description)
                        .bind(price)
                        .bind(&image_url)
                        .execute(&state.db)
                        .await;
                        
                        match result {
                            Ok(_) => {
                                println!("✅ Product saved to database!");
                                return Redirect::to("/admin?success=true");
                            }
                            Err(e) => println!("❌ Database error: {:?}", e),
                        }
                    }
                    Err(e) => println!("❌ R2 upload error: {:?}", e),
                }
            }
            Err(e) => println!("❌ R2 client error: {:?}", e),
        }
    }
    
    Redirect::to("/admin")
}

#[derive(Deserialize)]
pub struct DeleteProductForm {
    product_id: i32,
}

pub async fn delete_product_handler(
    State(state): State<AppState>,
    Form(form): Form<DeleteProductForm>,
) -> Redirect {
    if let Ok(product) = sqlx::query("SELECT icon FROM products WHERE id = $1")
        .bind(form.product_id)
        .fetch_one(&state.db)
        .await
    {
        let icon: String = product.get("icon");
        
        if let Ok(r2) = R2Client::new().await {
            let _ = r2.delete_image(&icon).await;
        }
    }
    
    let _ = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(form.product_id)
        .execute(&state.db)
        .await;
    
    Redirect::to("/admin")
}
