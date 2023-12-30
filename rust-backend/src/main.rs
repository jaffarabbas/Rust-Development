use actix_web::{get,post,patch,delete, App, HttpResponse, HttpServer, Responder, web::Json};
mod models;

use crate::models::pizzas::Pizza;
use validator::Validate;

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/post_pizzas")]
async fn buy_pizzas(body:Json<Pizza>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza = body.name.clone();
            HttpResponse::Ok().body(format!("{pizza}"))
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }
}

#[patch("/update_pizzas/{uuid}")]
async fn update_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[delete("/pizzas/{uuid}")]
async fn delete_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
    .service(get_pizzas)
    .service(buy_pizzas)
    .service(update_pizzas)
    .service(delete_pizzas))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

