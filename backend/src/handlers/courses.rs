use crate::models::NewCourse;
use crate::{models::Course, schema::courses, AppState};
use actix_web::{web, HttpResponse};
use common::CourseInfo;
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, QueryDsl};

// Retrieves information about a course.
pub async fn get_course(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    let slug = path.into_inner();

    let course: Course = courses::table
        .filter(courses::slug.eq(slug))
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

/// Creates a new course.
pub async fn create_course(data: web::Data<AppState>, json: web::Json<CourseInfo>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    match courses::table
        .filter(courses::name.eq(&json.name))
        .first::<Course>(&mut connection)
    {
        Ok(_) => return HttpResponse::Ok().finish(),
        Err(_) => (),
    };

    let course = NewCourse {
        slug: &json.name.to_lowercase().replace(' ', "-"),
        name: &json.name,
        description: &json.description,
        image: &json.image,
    };

    match diesel::insert_into(courses::table)
        .values(&course)
        .get_result::<Course>(&mut connection)
    {
        Ok(_) => return HttpResponse::Ok().finish(),
        Err(_) => (),
    };

    HttpResponse::Ok().finish()
}
