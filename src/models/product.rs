use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub icon: String,
}

impl Product {
    pub fn new(id: u64, name: String, description: String, price: f64, icon: String) -> Self {
        Self {
            id,
            name,
            description,
            price,
            icon,
        }
    }
}
