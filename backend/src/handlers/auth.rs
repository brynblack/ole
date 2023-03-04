use std::env;

use actix_web::{web, HttpResponse};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;

use crate::{
    models::{Account, LoginResponse, TempAcc},
    schema::accounts,
    server::AppState,
};

#[derive(Serialize)]
struct Claims {
    /// Expiration time (UTC)
    exp: u64,
    /// Subject (Who the token is given to)
    sub: String,
}

/// Generates a new JWT.
pub fn gen_token(sub: String) -> String {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let exp = jsonwebtoken::get_current_timestamp();

    let claims = Claims { sub, exp };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .unwrap()
}

/// Authenticates the user.
pub async fn login(data: web::Data<AppState>, json: web::Form<TempAcc>) -> HttpResponse {
    // Retrieve a database connection from the pool
    let mut connection = data.db_pool.get().unwrap();

    // Retrieve account from database
    let account = match accounts::table
        .filter(accounts::username.eq(&json.username))
        .first::<Account>(&mut connection)
    {
        Ok(account) => account,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    // Parse the already stored hash
    let parsed_hash = PasswordHash::new(&account.password).unwrap();

    // Verify if the given password matches the hash
    match Argon2::default().verify_password(json.password.as_bytes(), &parsed_hash) {
        Ok(_) => HttpResponse::Ok().json(web::Json(LoginResponse {
            token: gen_token(account.username),
        })),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

/// Revokes authentication for the user.
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().finish()
}
