use actix_web::{
    guard::Header,
    web::{self, ServiceConfig},
};

use crate::handlers::{accounts, auth, login};

/// Registers the API routes.
pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/accounts")
                    .route(web::post().to(accounts::create_acc))
                    .route(web::get().to(accounts::get_accounts)),
            )
            .service(
                web::resource("/users/{name}")
                    .guard(Header("content-type", "application/json"))
                    .route(web::get().to(auth::temp)),
            )
            .service(web::resource("/login").route(web::post().to(login::login)))
            .service(web::resource("/auth").route(web::post().to(auth::authorize))),
    );
}
