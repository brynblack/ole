use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::accounts;

#[derive(Debug, Queryable)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub pfp: String,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub pfp: &'a str,
}

#[derive(Deserialize)]
pub struct NewAcc {
    pub username: String,
    pub password: String,
    pub pfp: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct AccountData {
    pub pfp: String,
}
