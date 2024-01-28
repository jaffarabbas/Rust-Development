// // db.rs
// use sqlx::prelude::*;
// use crate::model::Products;
// use sqlx::Pool;

// pub fn pool() -> Pool<sqlx::AnyConnection> {
//     // Your database URL should come from your configuration
//     let database_url = std::env::var("DATABASE_URL")
//         .expect("DATABASE_URL not found in environment");

//     // Create a connection pool
//     Pool::connect(&database_url)
//         .expect("Failed to create database pool")
// }

// pub async fn get_products() -> Result<Vec<Products>, sqlx::Error> {
//     sqlx::query_as::<_, Products>("SELECT * FROM PRODUCT")
//         .fetch_all(pool())
//         .await
// }
