use std::env;

use actix_web::web::{Data, ServiceConfig};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use log::info;

pub struct AppState {
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
}

/// Builds the database connection pool.
pub fn database(cfg: &mut ServiceConfig) {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("initiating connection to database...");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = Pool::builder()
        .build(manager)
        .expect("failed to build pool");

    info!("connection successful");

    cfg.app_data(Data::new(AppState { db_pool }));
}
