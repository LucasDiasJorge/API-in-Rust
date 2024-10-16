use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Info {
    name: String,
    age: u8,
}

#[derive(Serialize)]
struct SuccessResponse {
    success: bool,
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    message: String,
}

async fn process_json(info: web::Json<Info>) -> impl Responder {

    if info.age == 0 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            message: "Age must be greater than 0".to_string(),
        });
    }

    if info.name.len() < 3 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            message: "Name must be at least 3 characters long".to_string(),
        });
    }

    let response_message = format!("Hello, {}! You are {} years old.", info.name, info.age);

    HttpResponse::Ok().json(SuccessResponse {
        success: true,
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
