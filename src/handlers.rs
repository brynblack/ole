use actix_web::{web, HttpResponse};
use diesel::RunQueryDsl;
use log::info;
use serde::Deserialize;

use crate::{
    database::AppData,
    models::{Account, NewAccount},
};

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("Logged in!")
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logged out!")
}

#[derive(Deserialize)]
pub struct TempAcc {
    pub username: String,
    pub password: String,
}

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

pub async fn remove_acc() -> HttpResponse {
    HttpResponse::Ok().body("Deleted Account!")
}

pub async fn get_accounts() -> HttpResponse {
    HttpResponse::Ok().body("Here are the registered accounts: ")
}
