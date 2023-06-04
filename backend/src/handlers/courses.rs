use actix_web::{web, HttpResponse};
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, QueryDsl};

use crate::models::{CourseInfo, NewCourse};
use crate::{models::Course, schema::courses, server::AppState};

pub async fn course(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let course_id = path.into_inner();

    let mut connection = data.db_pool.get().unwrap();

    let course: Course = courses::table
        .filter(courses::name.eq(course_id))
        .first::<Course>(&mut connection)
        .expect("course does not exist");

    HttpResponse::Ok().json(CourseInfo {
        name: course.name,
        description: course.description,
        image: course.image,
    })
}

/// Creates a new account.
pub async fn create_course(data: web::Data<AppState>, json: web::Json<CourseInfo>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    courses::table
        .filter(courses::name.eq(&json.name))
        .first::<Course>(&mut connection)
        .expect_err("course already exists");

    let course = NewCourse {
        name: &json.name,
        description: &json.description,
        image: &json.image,
    };

    diesel::insert_into(courses::table)
        .values(&course)
        .get_result::<Course>(&mut connection)
        .expect("error creating new course");

    HttpResponse::Ok().finish()
}
