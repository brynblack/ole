use diesel::prelude::*;

use crate::schema::{accounts, courses, lessons};

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
    pub slug: String,
    pub name: String,
    pub description: String,
    pub image: String,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = lessons)]
pub struct Lesson {
    pub id: i32,
    pub reference: String,
    pub slug: String,
    pub name: String,
    pub content: String,
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
    pub slug: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub image: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = lessons)]
pub struct NewLesson<'a> {
    pub reference: &'a str,
    pub slug: &'a str,
    pub name: &'a str,
    pub content: &'a str,
    pub image: &'a str,
}
