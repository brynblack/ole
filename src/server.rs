use std::env;

use actix_web::{App, HttpServer};
use log::info;

use crate::{database::database, routes::routes};

/// Starts the server.
pub async fn run() -> std::io::Result<()> {
    let app_port = env::var("APP_PORT")
        .expect("APP_PORT not set")
        .parse()
        .unwrap();

    info!("starting up server...");

    HttpServer::new(|| App::new().configure(database).configure(routes))
        .bind(("127.0.0.1", app_port))?
        .run()
        .await
}
