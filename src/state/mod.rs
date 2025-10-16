use crate::models::{Product, Cart};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AppState {
    pub products: Arc<Mutex<HashMap<u64, Product>>>,
    pub carts: Arc<Mutex<HashMap<String, Cart>>>,
    pub next_product_id: Arc<Mutex<u64>>,
}

impl AppState {
    pub fn new() -> Self {
        let mut products = HashMap::new();
        
        products.insert(
            1,
            Product::new(
                1,
                "Ethereal Essence Collection".to_string(),
                "Handcrafted artisan piece embodying spiritual harmony".to_string(),
                89.99,
                "🌿".to_string(),
            ),
        );
        
        products.insert(
            2,
            Product::new(
                2,
                "Sacred Geometry Set".to_string(),
                "Premium collection inspired by ancient wisdom".to_string(),
                129.99,
                "⚛️".to_string(),
            ),
        );

        Self {
            products: Arc::new(Mutex::new(products)),
            carts: Arc::new(Mutex::new(HashMap::new())),
            next_product_id: Arc::new(Mutex::new(3)),
        }
    }
}
