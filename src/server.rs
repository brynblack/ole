use std::env;

use actix_web::{web::Data, App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use log::info;

use crate::{database, routes::routes};
pub struct AppState {
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
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

    HttpServer::new(move || App::new().app_data(app_data.clone()).configure(routes))
        .bind(("127.0.0.1", app_port))?
        .run()
        .await
}
