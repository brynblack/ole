use actix_web::{web, HttpResponse};
use diesel::RunQueryDsl;
use log::info;
use serde::Deserialize;

use crate::{
    database::AppData,
    models::{Account, NewAccount},
};

/// Authenticates the user.
pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("Logged in!")
}

/// Revokes authentication for the user.
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logged out!")
}

#[derive(Deserialize)]
pub struct TempAcc {
    pub username: String,
    pub password: String,
}

/// Creates a new account with the given name and password.
///
/// The data is recieved through a REST post request, with JSON data
/// being sent describing the username and password of the new account to be created.
///
/// Returns a `HttpResponse` confirming that it was successful.
pub async fn create_acc(data: web::Data<AppData>, info: web::Json<TempAcc>) -> HttpResponse {
    use crate::schema::accounts;

    let mut connection = data.db_pool.get().unwrap();

    let acc = NewAccount {
        username: &info.username,
        password: &info.password,
    };

    info!("recieved account create request, creating account...");

    diesel::insert_into(accounts::table)
        .values(&acc)
        .get_result::<Account>(&mut connection)
        .expect("error creating new account");

    HttpResponse::Ok().body("Created Account!")
}

/// Deletes an account from the database of registered users.
pub async fn remove_acc() -> HttpResponse {
    HttpResponse::Ok().body("Deleted Account!")
}

/// Returns a space separated list of all the currently registured users.
pub async fn get_accounts(data: web::Data<AppData>) -> HttpResponse {
    use crate::schema::accounts;

    let mut connection = data.db_pool.get().unwrap();

    let rg_accounts = accounts::table.load::<Account>(&mut connection).unwrap();

    HttpResponse::Ok().body(format!(
        "Here are the registered accounts: {}",
        rg_accounts
            .into_iter()
            .map(|a| { format!("{} ", a.username) })
            .collect::<String>()
    ))
}
