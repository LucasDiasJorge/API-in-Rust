use bcrypt::{hash, verify};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::models::user::{LoginData, Claims};

const SECRET_KEY: &[u8] = b"supersecretkey";

// Função que gera o token JWT
pub fn generate_jwt(username: &str) -> jsonwebtoken::errors::Result<String> {
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
pub fn validate_jwt(token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )
}

// Função para registrar novos usuários (insere no banco)
pub async fn register_user(pool: &PgPool, username: &str, password: &str) -> Result<(), sqlx::Error> {
    let password_hash = hash(password, 4).unwrap();

    sqlx::query!("INSERT INTO users (username, password_hash) VALUES ($1, $2)", username, password_hash)
        .execute(pool)
        .await?;

    Ok(())
}

// Função para verificar o usuário
pub async fn verify_user(pool: &PgPool, username: &str, password: &str) -> Option<String> {
    if let Some(user) = sqlx::query!("SELECT password_hash FROM users WHERE username = $1", username)
        .fetch_optional(pool)
        .await
        .ok() {
        if let Some(user) = user {
            if verify(password, &user.password_hash).unwrap() {
                return Some(username.to_owned());
            }
        }
    }
    None
}
