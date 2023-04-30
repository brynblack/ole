use actix_web::web::{self, ServiceConfig};

use crate::handlers::{accounts, auth};

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
                web::resource("/auth")
                    .route(web::post().to(auth::login))
                    .route(web::delete().to(auth::logout)),
            ),
    );
}
