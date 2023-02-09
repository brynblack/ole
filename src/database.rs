use std::env;

use diesel::{Connection, PgConnection};
use log::{error, info};

pub fn establish_connection() -> PgConnection {
    dotenvy::dotenv().ok();

    info!("establishing connection to database...");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = PgConnection::establish(&database_url).unwrap_or_else(|_| {
        error!("database connection failed!");
        panic!("error connecting to {}", database_url)
    });

    info!("connection to database successful!");

    connection
}
