use actix_web::HttpResponse;
use diesel::RunQueryDsl;

use crate::{
    database::establish_connection,
    models::{Account, NewAccount},
};

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("Logged in!")
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logged out!")
}

pub async fn create_acc() -> HttpResponse {
    use crate::schema::accounts;

    let connection = &mut establish_connection();

    let acc = NewAccount {
        username: "Amy",
        password: "meow",
    };

    diesel::insert_into(accounts::table)
        .values(&acc)
        .get_result::<Account>(connection)
        .expect("error creating new account");

    HttpResponse::Ok().body("Created Account!")
}

pub async fn remove_acc() -> HttpResponse {
    HttpResponse::Ok().body("Deleted Account!")
}

pub async fn get_accounts() -> HttpResponse {
    HttpResponse::Ok().body("Here are the registered accounts: ")
}
