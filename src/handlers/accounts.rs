use std::env;

use actix_web::{web, HttpResponse};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    models::{AccToDelete, Account, NewAccount, TempAcc},
    schema::accounts,
    server::AppState,
};

/// Creates a new account.
pub async fn create_acc(data: web::Data<AppState>, json: web::Json<TempAcc>) -> HttpResponse {
    let username = &json.username;
    let password = &json.password;

    // Retrieve a database connection from the pool
    let mut connection = data.db_pool.get().unwrap();

    // Check if an account already exists with the given username
    accounts::table
        .filter(accounts::username.eq(username))
        .first::<Account>(&mut connection)
        .expect_err("account already exists");

    // Hash password
    let salt = env::var("PWD_SALT").expect("PWD_SALT must be set");
    let salt = SaltString::new(&salt).unwrap();
    let argon2 = Argon2::default();

    let password = &argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // Create new account
    let acc = NewAccount { username, password };

    // Insert account into table
    diesel::insert_into(accounts::table)
        .values(&acc)
        .get_result::<Account>(&mut connection)
        .expect("error creating new account");

    // Return OK response
    HttpResponse::Ok().finish()
}

/// Returns a space separated list of all the currently registered users.
pub async fn get_accounts(data: web::Data<AppState>) -> HttpResponse {
    // Retrieve a database connection from the pool
    let mut connection = data.db_pool.get().unwrap();

    // Retrieve a vector containing all of the accounts
    let rg_accounts = accounts::table
        .load::<Account>(&mut connection)
        .expect("error retrieving accounts");

    // Return OK response containing a list of the accounts
    HttpResponse::Ok().body(format!(
        "Here are the registered accounts: {}",
        rg_accounts
            .into_iter()
            .map(|a| { format!("{} ", a.username) })
            .collect::<String>()
    ))
}

/// Deletes an account.
pub async fn remove_acc(data: web::Data<AppState>, json: web::Json<AccToDelete>) -> HttpResponse {
    let id = json.id;

    // Retrieve a database connection from the pool
    let mut connection = data.db_pool.get().unwrap();

    // Delete the account by the given id
    diesel::delete(accounts::table.filter(accounts::id.eq(id)))
        .execute(&mut connection)
        .expect("error deleting account");

    // Return OK response
    HttpResponse::Ok().finish()
}
