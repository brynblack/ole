use crate::{
    models::{Account, NewAccount},
    schema::accounts,
    AppState,
};
use actix_web::{web, HttpResponse};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use common::{AccountData, NewAcc};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::env;

/// Creates a new account.
pub async fn create_acc(data: web::Data<AppState>, json: web::Json<NewAcc>) -> HttpResponse {
    // Retrieve a database connection from the pool
    let mut connection = data.db_pool.get().unwrap();

    // Extract data into variables for readability
    let username = &json.username;
    let password = &json.password;
    let pfp = &json.pfp;

    // Check if an account already exists with the given username
    match accounts::table
        .filter(accounts::username.eq(username))
        .first::<Account>(&mut connection)
    {
        Ok(_) => return HttpResponse::Ok().finish(),
        Err(_) => (),
    };

    // Hash password
    let salt = env::var("PWD_SALT").expect("PWD_SALT must be set");
    let salt = SaltString::new(&salt).unwrap();
    let argon2 = Argon2::default();

    let password = &argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // Create new account
    let acc = NewAccount {
        username,
        password,
        pfp,
    };

    // Insert account into table
    diesel::insert_into(accounts::table)
        .values(&acc)
        .get_result::<Account>(&mut connection)
        .expect("error creating new account");

    // Return OK response
    HttpResponse::Ok().finish()
}

pub async fn get_acc(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    let username = path.into_inner();

    let account: Account = accounts::table
        .filter(accounts::username.eq(username))
        .first::<Account>(&mut connection)
        .expect("account does not exist");

    HttpResponse::Ok().json(web::Json(AccountData { pfp: account.pfp }))
}
