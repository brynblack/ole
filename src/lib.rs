use std::sync::{Arc, Mutex};

use diesel::PgConnection;

pub mod handlers;
pub mod models;
pub mod schema;

pub struct AppState {
    connection: PgConnection,
}
