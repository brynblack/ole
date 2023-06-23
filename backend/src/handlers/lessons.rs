use crate::models::{Lesson, NewLesson};
use crate::schema::lessons;
use crate::AppState;
use actix_web::{web, HttpResponse};
use common::LessonInfo;
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, QueryDsl};

// Retrieves information about a lesson.
pub async fn get_lesson(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    let slug = path.into_inner();

    let lesson: Lesson = lessons::table
        .filter(lessons::slug.eq(slug))
        .first::<Lesson>(&mut connection)
        .expect("lesson does not exist");

    HttpResponse::Ok().json(LessonInfo {
        name: lesson.name,
        content: lesson.content,
        image: lesson.image,
    })
}

/// Gets the available lessons.
pub async fn get_lessons(data: web::Data<AppState>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    let lessons = lessons::table
        .load::<Lesson>(&mut connection)
        .expect("error retrieving lessons");

    let accs: Vec<LessonInfo> = lessons
        .into_iter()
        .map(|lsn| LessonInfo {
            name: lsn.name,
            content: lsn.content,
            image: lsn.image,
        })
        .collect();

    HttpResponse::Ok().json(accs)
}

/// Creates a new lesson.
pub async fn create_lesson(data: web::Data<AppState>, json: web::Json<LessonInfo>) -> HttpResponse {
    let mut connection = data.db_pool.get().unwrap();

    lessons::table
        .filter(lessons::name.eq(&json.name))
        .first::<Lesson>(&mut connection)
        .expect_err("lesson already exists");

    let lesson = NewLesson {
        slug: &json.name.to_lowercase().replace(' ', "-"),
        name: &json.name,
        content: &json.content,
        image: &json.image,
    };

    diesel::insert_into(lessons::table)
        .values(&lesson)
        .get_result::<Lesson>(&mut connection)
        .expect("error creating new lesson");

    HttpResponse::Ok().finish()
}
