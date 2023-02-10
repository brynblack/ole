use std::env;

use actix_web::web;
use diesel::{
    r2d2::{self, ConnectionManager, Pool},
    PgConnection,
};
use log::info;

/// Global variable storage for the app.
pub struct AppData {
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
}

/// Configures the database pool connection.
pub fn database(cfg: &mut web::ServiceConfig) {
    dotenvy::dotenv().ok();

    info!("establishing connection to database...");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build pool!");

    info!("successfully established database connection and pool");

    cfg.app_data(web::Data::new(AppData { db_pool }));
}
