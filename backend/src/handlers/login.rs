use super::Claims;
use crate::{models::Account, schema::accounts, AppState};
use actix_web::{web, HttpResponse};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{Duration, Utc};
use common::LoginRequest;
use common::LoginResponse;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use std::env;

/// Generates a new JWT.
pub fn gen_token(sub: String) -> String {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Set the expiration date for the token x seconds from now
    let exp = Utc::now()
        .checked_add_signed(Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims { sub, exp };

    jsonwebtoken::encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .unwrap()
}

/// Authenticates the user.
pub async fn login(data: web::Data<AppState>, json: web::Form<LoginRequest>) -> HttpResponse {
    // Retrieve a database connection from the pool
    let mut connection = data.db_pool.get().unwrap();

    // Retrieve account from database
    let account = match accounts::table
        .filter(accounts::username.eq(&json.username))
        .first::<Account>(&mut connection)
    {
        Ok(account) => account,
        Err(_) => return HttpResponse::Ok().finish(),
    };

    // Parse the already stored hash
    let parsed_hash = PasswordHash::new(&account.password).unwrap();

    // Verify if the given password matches the hash
    match Argon2::default().verify_password(json.password.as_bytes(), &parsed_hash) {
        Ok(_) => HttpResponse::Ok().json(web::Json(LoginResponse {
            token: gen_token(account.username),
        })),
        Err(_) => HttpResponse::Ok().finish(),
    }
}
