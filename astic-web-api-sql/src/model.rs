use serde::{Serialize,Deserialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Products {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image: String,
    pub quantity: i32,
    pub created_on: String,
    pub cid: i32,
    pub product_status: String,
}