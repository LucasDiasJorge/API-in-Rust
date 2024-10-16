use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use bcrypt::{hash, verify};
use std::time::{SystemTime, UNIX_EPOCH};

// Chave secreta para assinatura dos tokens JWT
const SECRET_KEY: &[u8] = b"supersecretkey";

// Estrutura dos dados do usuário para autenticação
#[derive(Debug, Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

// Estrutura do token JWT (Claims)
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // O sujeito (nome do usuário) do token
    exp: usize,   // Data de expiração do token
}

// Estruturas para as respostas JSON
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

// Função que gera o token JWT
fn generate_jwt(username: &str) -> jsonwebtoken::errors::Result<String> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() + 60 * 60;  // 1 hora de validade

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
}

// Função para validar o token JWT
fn validate_jwt(token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )
}

// Função que processa o JSON (validando o token JWT)
async fn process_json(req: HttpRequest, info: web::Json<Info>) -> impl Responder {
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

// Manipulador para o login (gera o JWT)
async fn login(info: web::Json<LoginData>) -> impl Responder {
    // Usuário e senha "hardcoded" para exemplo (substituir por BD)
    let stored_password_hash = hash("password123", 4).unwrap();  // senha hashada

    if info.username == "user" && verify(&info.password, &stored_password_hash).unwrap() {
        match generate_jwt(&info.username) {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(login))  // Rota para login
            .route("/json", web::post().to(process_json))  // Rota protegida
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
