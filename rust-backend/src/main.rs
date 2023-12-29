use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::io;

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_pizzas))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

