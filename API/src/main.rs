use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

mod models;
mod services;
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Carrega o arquivo .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Cria um pool de conexões com o banco de dados
    let pool = PgPool::connect(&database_url).await.expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Adiciona o pool de conexões no App
            .route("/login", web::post().to(controllers::auth::login))  // Rota para login
            .route("/register", web::post().to(controllers::auth::register))  // Rota para registro
            .route("/json", web::post().to(controllers::json::process_json))  // Rota protegida
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
