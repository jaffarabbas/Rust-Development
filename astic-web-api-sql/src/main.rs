// main.rs
use actix_web::{get, web, App, HttpServer,HttpResponse, Responder};
use sqlx::Pool;

mod db;
mod model;

#[get("/products")]
async fn get_products() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
    // match db::get_products().await {
    //     Ok(products) => web::Json(products),
    //     Err(_) => web::HttpResponse::InternalServerError(),
    // }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Read configuration from environment variable
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable not found");

    // Create connection pool
    let pool = sqlx::pool::Pool::connect(&database_url)
        .await
        .expect("Failed to create connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_products)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
