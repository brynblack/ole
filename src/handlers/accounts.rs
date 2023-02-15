use actix_web::{web, HttpResponse};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    models::{AccToDelete, Account, NewAccount, TempAcc},
    schema::accounts,
    server::AppState,
};

use log::info;

/// Creates a new account with the given name and password.
///
/// The data is recieved through a REST post request, with JSON data
/// being sent describing the username and password of the new account to be created.
///
/// Returns a `HttpResponse` confirming that it was successful.
pub async fn create_acc(data: web::Data<AppState>, info: web::Json<TempAcc>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    info!("recieved account create request, creating account...");

    let salt = SaltString::new("meowmeowmeowmeowmeowmeowmeowmeowmeowmeowmeowmeowmeowmeow").unwrap();
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(info.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let acc = NewAccount {
        username: &info.username,
        password: &hash,
    };

    diesel::insert_into(accounts::table)
        .values(&acc)
        .get_result::<Account>(&mut connection)
        .expect("error creating new account");

    info!("created account");

    HttpResponse::Ok().body("Created Account!")
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
