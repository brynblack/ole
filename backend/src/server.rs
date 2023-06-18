use crate::handlers::accounts::create_acc;
use crate::AppState;
use crate::{database, routes::routes};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::web::Json;
use actix_web::{web::Data, App, HttpServer};
use common::NewAcc;
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

    // Create default admin account if first time setup
    create_acc(
        app_data.clone(),
        Json(NewAcc {
            username: "admin".to_string(),
            password: "password".to_string(),
            pfp: "https://upload.wikimedia.org/wikipedia/commons/b/b5/Windows_10_Default_Profile_Picture.svg".to_string(),
        }),
    )
    .await;

    // Initialise and run a new HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080")
                    .allowed_origin("http://127.0.0.1:8080")
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
