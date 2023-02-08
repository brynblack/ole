use actix_web::{web, App, HttpServer};
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use ole::{handlers, AppState};
use std::env;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(|| {
                let connection = establish_connection();

                AppState { connection }
            }))
            .service(
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
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
