use axum::{
    extract::{Query, State},
    response::{Html, Redirect},
    Form,
};
use axum_extra::extract::Multipart;
use serde::Deserialize;
use crate::models::Product;
use crate::state::AppState;
use crate::templates::base_layout;

#[derive(Deserialize)]
pub struct AdminQuery {
    success: Option<String>,
}

pub async fn admin_handler(State(state): State<AppState>, Query(query): Query<AdminQuery>) -> Html<String> {
    let products = state.products.lock().unwrap();
    
    let success_modal = if query.success.is_some() {
        r##"<div id="success-toast" style="position: fixed; top: 20px; right: 20px; background: linear-gradient(135deg, #27ae60, #2ecc71); color: white; padding: 20px 30px; border-radius: 12px; box-shadow: 0 8px 30px rgba(39, 174, 96, 0.4); z-index: 1000; animation: slideIn 0.5s ease; border-left: 5px solid #27ae60;">
            <div style="display: flex; align-items: center; gap: 15px;">
                <div style="font-size: 2rem;">✓</div>
                <div>
                    <strong style="display: block; font-size: 1.1rem; margin-bottom: 5px;">Product Added!</strong>
                    <span style="font-size: 0.9rem; opacity: 0.9;">One love, one heart ✨</span>
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
    for product in products.values() {
        let display_icon = if product.icon.starts_with("data:image") {
            format!(r##"<img src="{}" style="width: 70px; height: 70px; object-fit: cover; border-radius: 50%; margin-right: 15px; border: 2px solid var(--secondary-gold);" alt="{}">"##, product.icon, product.name)
        } else {
            format!(r##"<span style="font-size: 2.5rem; margin-right: 15px;">{}</span>"##, product.icon)
        };
        
        products_list.push_str(&format!(
            r##"<div style="background: rgba(255, 255, 255, 0.03); padding: 20px; border-radius: 12px; margin-bottom: 15px; display: flex; justify-content: space-between; align-items: center; border: 1px solid rgba(212, 175, 55, 0.2); transition: all 0.3s ease;" onmouseover="this.style.borderColor='rgba(243, 156, 18, 0.5)'; this.style.background='rgba(255, 255, 255, 0.05)';" onmouseout="this.style.borderColor='rgba(212, 175, 55, 0.2)'; this.style.background='rgba(255, 255, 255, 0.03)';">
                <div style="display: flex; align-items: center; flex: 1;">
                    {}
                    <div>
                        <h4 style="color: var(--secondary-gold); margin-bottom: 8px; font-family: 'Cinzel', serif;">{}</h4>
                        <p style="color: var(--light); opacity: 0.8; font-size: 0.9rem;">{}</p>
                        <p style="font-weight: bold; color: var(--rasta-gold); margin-top: 8px; font-size: 1.1rem;">${}</p>
                    </div>
                </div>
                <form method="post" action="/admin/products/delete">
                    <input type="hidden" name="product_id" value="{}">
                    <button type="submit" style="padding: 10px 20px; background: linear-gradient(135deg, #e74c3c, #c0392b); color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 600; transition: all 0.3s ease;" onmouseover="this.style.transform='scale(1.05)'; this.style.boxShadow='0 4px 15px rgba(231, 76, 60, 0.4)';" onmouseout="this.style.transform=''; this.style.boxShadow='';">Delete</button>
                </form>
            </div>"##,
            display_icon, product.name, product.description, product.price, product.id
        ));
    }
    
    let content = format!(
        r##"{}
        <div class="rasta-accent" style="margin-bottom: 30px;"></div>
        
        <h2 style="color: var(--secondary-gold); font-size: 2.5rem; margin-bottom: 30px; font-family: 'Cinzel', serif; text-align: center;">
            Admin Panel
        </h2>
        
        <div style="background: rgba(255, 255, 255, 0.03); padding: 35px; border-radius: 15px; margin-bottom: 40px; border: 1px solid rgba(212, 175, 55, 0.3); box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);">
            <div class="rasta-accent" style="margin-bottom: 25px;"></div>
            <h3 style="color: var(--secondary-gold); margin-bottom: 25px; font-family: 'Cinzel', serif; font-size: 1.5rem;">Add New Product</h3>
            <form id="product-form" method="post" action="/admin/products" enctype="multipart/form-data">
                <div style="margin-bottom: 20px;">
                    <label style="display: block; margin-bottom: 10px; color: var(--rasta-gold); font-weight: 600; font-size: 0.95rem; text-transform: uppercase; letter-spacing: 1px;">Product Name</label>
                    <input type="text" name="name" id="product-name" required style="width: 100%; padding: 14px; border: 2px solid rgba(243, 156, 18, 0.3); border-radius: 10px; background: rgba(255, 255, 255, 0.05); color: var(--light); font-size: 1rem; transition: all 0.3s ease;" onfocus="this.style.borderColor='var(--rasta-gold)'; this.style.background='rgba(255, 255, 255, 0.08)';" onblur="this.style.borderColor='rgba(243, 156, 18, 0.3)'; this.style.background='rgba(255, 255, 255, 0.05)';">
                </div>
                
                <div style="margin-bottom: 20px;">
                    <label style="display: block; margin-bottom: 10px; color: var(--rasta-gold); font-weight: 600; font-size: 0.95rem; text-transform: uppercase; letter-spacing: 1px;">Description</label>
                    <textarea name="description" id="product-desc" required style="width: 100%; padding: 14px; border: 2px solid rgba(243, 156, 18, 0.3); border-radius: 10px; background: rgba(255, 255, 255, 0.05); color: var(--light); font-size: 1rem; min-height: 120px; resize: vertical; transition: all 0.3s ease; font-family: 'Raleway', sans-serif;" onfocus="this.style.borderColor='var(--rasta-gold)'; this.style.background='rgba(255, 255, 255, 0.08)';" onblur="this.style.borderColor='rgba(243, 156, 18, 0.3)'; this.style.background='rgba(255, 255, 255, 0.05)';"></textarea>
                </div>
                
                <div style="margin-bottom: 20px;">
                    <label style="display: block; margin-bottom: 10px; color: var(--rasta-gold); font-weight: 600; font-size: 0.95rem; text-transform: uppercase; letter-spacing: 1px;">Price ($)</label>
                    <input type="number" name="price" id="product-price" step="0.01" required style="width: 100%; padding: 14px; border: 2px solid rgba(243, 156, 18, 0.3); border-radius: 10px; background: rgba(255, 255, 255, 0.05); color: var(--light); font-size: 1rem; transition: all 0.3s ease;" onfocus="this.style.borderColor='var(--rasta-gold)'; this.style.background='rgba(255, 255, 255, 0.08)';" onblur="this.style.borderColor='rgba(243, 156, 18, 0.3)'; this.style.background='rgba(255, 255, 255, 0.05)';">
                </div>
                
                <div style="margin-bottom: 25px;">
                    <label style="display: block; margin-bottom: 10px; color: var(--rasta-gold); font-weight: 600; font-size: 0.95rem; text-transform: uppercase; letter-spacing: 1px;">Product Image</label>
                    <input type="file" id="image-input" name="image" accept="image/*" required style="width: 100%; padding: 14px; border: 2px solid rgba(243, 156, 18, 0.3); border-radius: 10px; background: rgba(255, 255, 255, 0.05); color: var(--light); font-size: 1rem; transition: all 0.3s ease;" onfocus="this.style.borderColor='var(--rasta-gold)';" onblur="this.style.borderColor='rgba(243, 156, 18, 0.3)';">
                    <div id="file-info" style="margin-top: 12px; color: var(--rasta-green); font-size: 0.9rem; font-weight: 500;"></div>
                </div>
                
                <button id="submit-btn" type="submit" style="width: 100%; padding: 16px; background: linear-gradient(135deg, var(--secondary-gold) 0%, var(--rasta-gold) 100%); color: var(--primary); border: none; border-radius: 10px; font-weight: 700; cursor: pointer; text-transform: uppercase; letter-spacing: 2px; font-size: 1rem; transition: all 0.3s ease; box-shadow: 0 4px 15px rgba(243, 156, 18, 0.3);" onmouseover="this.style.transform='scale(1.02)'; this.style.boxShadow='0 6px 25px rgba(243, 156, 18, 0.5)';" onmouseout="this.style.transform=''; this.style.boxShadow='0 4px 15px rgba(243, 156, 18, 0.3)';">
                    Add Product
                </button>
                
                <div id="loading" style="display: none; text-align: center; margin-top: 25px;">
                    <div style="display: inline-block; width: 50px; height: 50px; border: 5px solid rgba(243, 156, 18, 0.2); border-top-color: var(--rasta-gold); border-radius: 50%; animation: spin 1s linear infinite;"></div>
                    <p style="color: var(--rasta-gold); margin-top: 15px; font-weight: 600; letter-spacing: 1px;">Uploading... One love 🌿</p>
                </div>
            </form>
            
            <style>
                @keyframes spin {{
                    to {{ transform: rotate(360deg); }}
                }}
            </style>
            
            <script>
                const fileInput = document.getElementById('image-input');
                const fileInfo = document.getElementById('file-info');
                const form = document.getElementById('product-form');
                const submitBtn = document.getElementById('submit-btn');
                const loading = document.getElementById('loading');
                
                fileInput.addEventListener('change', (e) => {{
                    const file = e.target.files[0];
                    if (file) {{
                        const sizeMB = (file.size / 1024 / 1024).toFixed(2);
                        fileInfo.innerHTML = `<span style="color: var(--rasta-green);">✓</span> Selected: ${{file.name}} (${{sizeMB}} MB)`;
                        
                        if (file.size > 10 * 1024 * 1024) {{
                            fileInfo.style.color = 'var(--rasta-red)';
                            fileInfo.innerHTML = `<span style="color: var(--rasta-red);">⚠</span> Large file (${{sizeMB}} MB) - May take time to upload`;
                        }}
                    }}
                }});
                
                form.addEventListener('submit', (e) => {{
                    submitBtn.disabled = true;
                    loading.style.display = 'block';
                }});
            </script>
        </div>
        
        <div class="rasta-accent" style="margin: 40px 0;"></div>
        
        <h3 style="color: var(--secondary-gold); margin-bottom: 25px; font-family: 'Cinzel', serif; font-size: 1.8rem; text-align: center;">
            Current Products ({})
        </h3>
        <div style="margin-top: 20px;">
            {}
        </div>"##,
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
    let mut price = 0.0;
    let mut image_base64 = String::new();
    
    println!("Starting multipart processing...");
    
    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = match field.name() {
            Some(n) => {
                println!("Processing field: {}", n);
                n.to_string()
            },
            None => {
                println!("Field with no name, skipping");
                continue;
            }
        };
        
        match field_name.as_str() {
            "name" => {
                match field.text().await {
                    Ok(text) => {
                        println!("Got name: {}", text);
                        name = text;
                    },
                    Err(e) => println!("Error reading name: {:?}", e),
                }
            }
            "description" => {
                match field.text().await {
                    Ok(text) => {
                        println!("Got description: {} chars", text.len());
                        description = text;
                    },
                    Err(e) => println!("Error reading description: {:?}", e),
                }
            }
            "price" => {
                match field.text().await {
                    Ok(text) => {
                        price = text.parse().unwrap_or(0.0);
                        println!("Got price: {}", price);
                    },
                    Err(e) => println!("Error reading price: {:?}", e),
                }
            }
            "image" => {
                match field.bytes().await {
                    Ok(data) => {
                        println!("Got image: {} bytes", data.len());
                        image_base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &data);
                        println!("Encoded to base64: {} chars", image_base64.len());
                    },
                    Err(e) => {
                        println!("Error reading image: {:?}", e);
                    }
                }
            }
            _ => {
                println!("Unknown field: {}", field_name);
            }
        }
    }
    
    println!("Multipart processing complete");
    println!("Name: {}, Desc length: {}, Price: {}, Image length: {}", 
             name, description.len(), price, image_base64.len());
    
    if !name.is_empty() && !description.is_empty() && !image_base64.is_empty() && price > 0.0 {
        let mut products = state.products.lock().unwrap();
        let mut next_id = state.next_product_id.lock().unwrap();
        
        let new_product = Product::new(
            *next_id,
            name,
            description,
            price,
            format!("data:image/jpeg;base64,{}", image_base64),
        );
        
        println!("Adding product with ID: {}", *next_id);
        products.insert(*next_id, new_product);
        *next_id += 1;
        println!("Product added successfully!");
        
        Redirect::to("/admin?success=true")
    } else {
        println!("Validation failed - missing required fields");
        Redirect::to("/admin")
    }
}

#[derive(Deserialize)]
pub struct DeleteProductForm {
    product_id: u64,
}

pub async fn delete_product_handler(
    State(state): State<AppState>,
    Form(form): Form<DeleteProductForm>,
) -> Redirect {
    let mut products = state.products.lock().unwrap();
    products.remove(&form.product_id);
    println!("Deleted product: {}", form.product_id);
    
    Redirect::to("/admin")
}
