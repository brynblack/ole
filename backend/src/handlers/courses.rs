use crate::models::NewCourse;
use crate::{models::Course, schema::courses, server::AppState};
use actix_web::{web, HttpResponse};
use common::CourseInfo;
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, QueryDsl};

// Retrieves information about a course.
pub async fn get_course(data: web::Data<AppState>, path: web::Path<i32>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    let course_id = path.into_inner();

    let course: Course = courses::table
        .filter(courses::id.eq(course_id))
        .first::<Course>(&mut connection)
        .expect("course does not exist");

    HttpResponse::Ok().json(CourseInfo {
        name: course.name,
        description: course.description,
        image: course.image,
    })
}

pub async fn get_courses(data: web::Data<AppState>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    let courses = courses::table
        .load::<Course>(&mut connection)
        .expect("error retrieving courses");

    let accs: Vec<CourseInfo> = courses
        .into_iter()
        .map(|acc| CourseInfo {
            name: acc.name,
            description: acc.description,
            image: acc.image,
        })
        .collect();

    HttpResponse::Ok().json(accs)
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
