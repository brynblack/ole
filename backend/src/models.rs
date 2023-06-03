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
pub struct LoginAcc {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct NewAcc {
    pub username: String,
    pub password: String,
    pub pfp: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pfp: String,
}
