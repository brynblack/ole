use super::Claims;
use crate::models::{Account, AccountData};
use crate::schema::accounts;
use crate::{models::AuthRequest, server::AppState};
use actix_web::{web, HttpResponse};
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, QueryDsl};
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

pub async fn temp(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let username = path.into_inner();

    // Retrieve a database connection from the pool
    let mut connection = data.db_pool.get().unwrap();

    let account: Account = accounts::table
        .filter(accounts::username.eq(username))
        .first::<Account>(&mut connection)
        .expect("account does not exist");

    HttpResponse::Ok().json(web::Json(AccountData { pfp: account.pfp }))
}
