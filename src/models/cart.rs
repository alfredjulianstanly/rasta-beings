use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: u64,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub items: Vec<CartItem>,
}

impl Cart {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    
    pub fn add_item(&mut self, product_id: u64) {
        if let Some(item) = self.items.iter_mut().find(|i| i.product_id == product_id) {
            item.quantity += 1;
        } else {
            self.items.push(CartItem {
                product_id,
                quantity: 1,
            });
        }
    }
}
