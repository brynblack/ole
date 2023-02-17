use std::env;

use actix_web::{web::Data, App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use log::info;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use crate::{database, routes::routes};

pub struct AppState {
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
}

fn ssl_builder() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    builder
}

/// Starts the server.
pub async fn run() -> std::io::Result<()> {
    let app_port = env::var("APP_PORT")
        .expect("APP_PORT must be set")
        .parse()
        .unwrap();

    info!("starting up server...");

    let db_pool = database::db_connect();
    let app_data = Data::new(AppState { db_pool });
    let builder = ssl_builder();

    HttpServer::new(move || App::new().app_data(app_data.clone()).configure(routes))
        .bind_openssl(("127.0.0.1", app_port), builder)?
        .run()
        .await
}
