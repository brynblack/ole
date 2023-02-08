use actix_web::{web, HttpResponse};
use diesel::{PgConnection, RunQueryDsl};

use crate::{models::NewAccount, AppState};

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("Logged in!")
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logged out!")
}

pub async fn create_acc(data: web::Data<AppState>) -> HttpResponse {
    let new_account = NewAccount {
        username: "Amy",
        password: "test123",
    };

    use crate::schema::accounts;

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result::<PgConnection>(data.connection)
        .unwrap();

    HttpResponse::Ok().body("Created Account!")
}

pub async fn remove_acc() -> HttpResponse {
    HttpResponse::Ok().body("Deleted Account!")
}
