use crate::handlers::{accounts, auth, courses, lessons, login};
use actix_web::web::{self, ServiceConfig};

/// Registers the API routes.
pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/accounts", web::post().to(accounts::create_acc))
            .route("/accounts/{name}", web::get().to(accounts::get_acc))
            .route("/courses", web::post().to(courses::create_course))
            .route("/courses", web::get().to(courses::get_courses))
            .route("/courses/{id}", web::get().to(courses::get_course))
            .route("/lessons", web::post().to(lessons::create_lesson))
            .route("/lessons", web::get().to(lessons::get_lessons))
            .route("/lessons/{id}", web::get().to(lessons::get_lesson))
            .route("/login", web::post().to(login::login))
            .route("/auth", web::post().to(auth::authorize)),
    );
}
