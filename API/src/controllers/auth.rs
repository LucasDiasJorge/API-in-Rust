use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::user::{LoginData, SuccessResponse, ErrorResponse};
use crate::services::auth_service::{generate_jwt, verify_user, register_user};

// Função que processa o login
pub async fn login(pool: web::Data<PgPool>, info: web::Json<LoginData>) -> impl Responder {
    if let Some(username) = verify_user(pool.get_ref(), &info.username, &info.password).await {
        match generate_jwt(&username) {
            Ok(token) => HttpResponse::Ok().json(SuccessResponse {
                success: true,
                message: format!("Bearer {}", token),
            }),
            Err(_) => HttpResponse::InternalServerError().body("Could not create token"),
        }
    } else {
        HttpResponse::Unauthorized().json(ErrorResponse {
            success: false,
            message: "Invalid credentials".to_string(),
        })
    }
}

// Função que processa o registro
pub async fn register(pool: web::Data<PgPool>, info: web::Json<LoginData>) -> impl Responder {
    if let Err(_) = register_user(pool.get_ref(), &info.username, &info.password).await {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            message: "Failed to register user".to_string(),
        });
    }

    HttpResponse::Ok().json(SuccessResponse {
        success: true,
        message: "User registered successfully".to_string(),
    })
}
