use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub icon: String,
    
    pub sku: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub sizes: Option<Vec<String>>,
    pub colors: Option<Vec<String>>,
    
    pub mrp: Option<Decimal>,
    pub discount_percent: Option<Decimal>,
    pub cost_price: Option<Decimal>,
    pub profit_margin: Option<Decimal>,
    pub tax_percent: Option<Decimal>,
    
    pub stock_quantity: Option<i32>,
    pub supplier_name: Option<String>,
    pub supplier_contact: Option<String>,
    pub purchase_date: Option<String>,
    pub invoice_number: Option<String>,
}
