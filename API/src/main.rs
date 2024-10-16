use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Info {
    name: String,
    age: u8,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

async fn process_json(info: web::Json<Info>) -> impl Responder {
    let response_message = format!("Hello, {}! You are {} years old.", info.name, info.age);

    HttpResponse::Ok().json(Response {
        message: response_message,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/json", web::post().to(process_json))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
