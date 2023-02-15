use std::env;

use actix_web::{web, HttpResponse};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{
    models::{Account, LoginResponse, TempAcc},
    schema::accounts,
    server::AppState,
};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    /// Expiration time (UTC)
    exp: u64,
    /// Subject (Who the token is given to)
    sub: String,
}

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
pub async fn login(data: web::Data<AppState>, info: web::Json<TempAcc>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    let account = accounts::table
        .filter(accounts::username.eq(&info.username))
        .first::<Account>(&mut connection)
        .unwrap();

    let parsed_hash = PasswordHash::new(&account.password).unwrap();

    if Argon2::default()
        .verify_password(info.password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        HttpResponse::Ok().json(web::Json(LoginResponse {
            token: gen_token(account.username),
        }))
    } else {
        HttpResponse::BadRequest().body("Invalid credentials!")
    }
}

/// Revokes authentication for the user.
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logged out!")
}
