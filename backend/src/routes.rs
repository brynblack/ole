use crate::handlers::{accounts, auth, courses, login};
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
            .route("/login", web::post().to(login::login))
            .route("/auth", web::post().to(auth::authorize)),
    );
}
