use actix_web::{web, App, HttpResponse, HttpServer};

async fn login() -> HttpResponse {
    HttpResponse::Ok().body("Logged in!")
}

async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logged out!")
}

async fn create_acc() -> HttpResponse {
    HttpResponse::Ok().body("Created Account!")
}

async fn remove_acc() -> HttpResponse {
    HttpResponse::Ok().body("Deleted Account!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(
                    web::resource("/auth")
                        .route(web::post().to(login))
                        .route(web::delete().to(logout)),
                )
                .service(
                    web::resource("/accounts")
                        .route(web::post().to(create_acc))
                        .route(web::delete().to(remove_acc)),
                ),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
