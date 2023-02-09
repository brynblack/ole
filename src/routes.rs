use crate::handlers;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/auth")
                    .route(web::post().to(handlers::login))
                    .route(web::delete().to(handlers::logout)),
            )
            .service(
                web::resource("/accounts")
                    .route(web::post().to(handlers::create_acc))
                    .route(web::delete().to(handlers::remove_acc)),
            ),
    );
}
