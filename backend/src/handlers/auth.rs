use super::Claims;
use actix_web::{web, HttpResponse};
use common::AuthRequest;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use std::env;

/// Validates a given token.
pub async fn authorize(json: web::Json<AuthRequest>) -> HttpResponse {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let decoded = jsonwebtoken::decode::<Claims>(
        &json.token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    );

    match decoded {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}
