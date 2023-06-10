use crate::AppState;
use crate::{database, routes::routes};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web::Data, App, HttpServer};
use log::info;
use std::env;

/// Starts the server.
pub async fn run() -> std::io::Result<()> {
    let app_port = env::var("APP_PORT")
        .expect("APP_PORT must be set")
        .parse()
        .unwrap();

    info!("starting up server...");

    let db_pool = database::db_connect();
    let app_data = Data::new(AppState { db_pool });
    let builder = crate::ssl_builder();

    // Initialise and run a new HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(app_data.clone())
            .configure(routes)
    })
    .bind_openssl(("localhost", app_port), builder)?
    .run()
    .await
}
