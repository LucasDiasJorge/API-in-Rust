use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use bcrypt::{hash, verify};
use sqlx::{PgPool, query};
use dotenv::dotenv;
use std::env;
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

// Manipulador para o login (persiste dados e gera o JWT)
async fn login(pool: web::Data<PgPool>, info: web::Json<LoginData>) -> impl Responder {
    // Verifique se o usuário existe no banco
    let user = sqlx::query!("SELECT password_hash FROM users WHERE username = $1", info.username)
        .fetch_optional(pool.get_ref())
        .await
        .expect("Error fetching user");

    if let Some(user) = user {
        // Verifique a senha
        if verify(&info.password, &user.password_hash).unwrap() {
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
    } else {
        HttpResponse::Unauthorized().json(ErrorResponse {
            success: false,
            message: "Invalid credentials".to_string(),
        })
    }
}

// Função para registrar novos usuários (insere no banco)
async fn register(pool: web::Data<PgPool>, info: web::Json<LoginData>) -> impl Responder {
    let password_hash = hash(&info.password, 4).unwrap();

    let result = sqlx::query!("INSERT INTO users (username, password_hash) VALUES ($1, $2)", info.username, password_hash)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse {
            success: true,
            message: "User registered successfully".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            message: "Failed to register user".to_string(),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Carrega o arquivo .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Cria um pool de conexões com o banco de dados
    let pool = PgPool::connect(&database_url).await.expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Adiciona o pool de conexões no App
            .route("/login", web::post().to(login))  // Rota para login
            .route("/register", web::post().to(register))  // Rota para registro
            .route("/json", web::post().to(process_json))  // Rota protegida
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
