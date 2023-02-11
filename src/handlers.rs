use std::env;
use std::time::SystemTime;

use actix_web::{web, HttpResponse};
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};

use jsonwebtoken::{Algorithm, EncodingKey, Header};
use log::info;
use serde::{Deserialize, Serialize};

use crate::models::{AccToDelete, TempAcc};
use crate::schema::accounts;
use crate::{
    database::AppState,
    models::{Account, NewAccount},
};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(uid: &str) -> String {
    let expiration = SystemTime::now().elapsed().unwrap().as_secs();

    let claims = Claims {
        sub: uid.to_owned(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);
    jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set")
                .as_bytes(),
        ),
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

    if info.password == account.password {
        HttpResponse::Ok().body(create_jwt(&account.username))
    } else {
        HttpResponse::BadRequest().body("Invalid credentials!")
    }
}

/// Revokes authentication for the user.
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logged out!")
}

/// Creates a new account with the given name and password.
///
/// The data is recieved through a REST post request, with JSON data
/// being sent describing the username and password of the new account to be created.
///
/// Returns a `HttpResponse` confirming that it was successful.
pub async fn create_acc(data: web::Data<AppState>, info: web::Json<TempAcc>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    info!("recieved account create request, creating account...");

    let acc = NewAccount {
        username: &info.username,
        password: &info.password,
    };

    diesel::insert_into(accounts::table)
        .values(&acc)
        .get_result::<Account>(&mut connection)
        .expect("error creating new account");

    info!("created account");

    HttpResponse::Ok().body("Created Account!")
}

/// Deletes an account from the database of registered users.
///
/// The data is recieved through a REST post request, with JSON data
/// being sent describing the id of the account to be deleted.
///
/// Returns a `HttpResponse` confirming that it was successful.
pub async fn remove_acc(data: web::Data<AppState>, info: web::Json<AccToDelete>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    info!("recieved account delete request, deleting account...");

    diesel::delete(accounts::table.filter(accounts::id.eq(info.id)))
        .execute(&mut connection)
        .expect("error deleting account");

    info!("deleted account");

    HttpResponse::Ok().body("Deleted Account!")
}

/// Returns a space separated list of all the currently registured users.
pub async fn get_accounts(data: web::Data<AppState>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    info!("retrieving currently registered accounts...");

    let rg_accounts = accounts::table
        .load::<Account>(&mut connection)
        .expect("error retrieving accounts");

    info!("retrieved registered accounts");

    HttpResponse::Ok().body(format!(
        "Here are the registered accounts: {}",
        rg_accounts
            .into_iter()
            .map(|a| { format!("{} ", a.username) })
            .collect::<String>()
    ))
}
