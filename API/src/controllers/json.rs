use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::models::user::{SuccessResponse, ErrorResponse};
use crate::models::json::Info;
use crate::services::auth_service::validate_jwt;

// Função que processa o JSON (validando o token JWT)
pub async fn process_json(req: HttpRequest, info: web::Json<Info>) -> impl Responder {
    // Validação do token JWT
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];  // Remove "Bearer " do início

                // Validar o token JWT
                match validate_jwt(token) {
                    Ok(_) => {
                        // Se o token for válido, continuar com a lógica de validação do JSON
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

                        return HttpResponse::Ok().json(SuccessResponse {
                            success: true,
                            message: response_message,
                        })
                    }
                    Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse {
                        success: false,
                        message: "Invalid token".to_string(),
                    }),
                }
            }
        }
    }

    HttpResponse::Unauthorized().json(ErrorResponse {
        success: false,
        message: "No token provided".to_string(),
    })
}
