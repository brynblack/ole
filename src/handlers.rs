use actix_web::{web, HttpResponse};
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};
use log::info;
use serde::Deserialize;

use crate::{
    database::AppState,
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
pub async fn create_acc(data: web::Data<AppState>, info: web::Json<TempAcc>) -> HttpResponse {
    use crate::schema::accounts;

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

#[derive(Deserialize)]
pub struct AccToDelete {
    pub id: i32,
}

/// Deletes an account from the database of registered users.
pub async fn remove_acc(data: web::Data<AppState>, info: web::Json<AccToDelete>) -> HttpResponse {
    use crate::schema::accounts;

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
    use crate::schema::accounts;

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
