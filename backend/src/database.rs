use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use std::env;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// Builds the database connection pool.
pub fn db_connect() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("initiating connection to database...");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = Pool::builder()
        .build(manager)
        .expect("failed to build pool");

    info!("connection successful, running migrations");

    db_pool
        .get()
        .unwrap()
        .run_pending_migrations(MIGRATIONS)
        .expect("failed to run migrations");

    info!("migrations successful");

    db_pool
}
