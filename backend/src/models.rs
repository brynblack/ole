use diesel::prelude::*;

use crate::schema::{accounts, courses};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub pfp: String,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub pfp: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = courses)]
pub struct NewCourse<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub image: &'a str,
}
