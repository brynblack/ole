use crate::schema::accounts;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize)]
pub struct AccToDelete {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct TempAcc {
    pub username: String,
    pub password: String,
}
