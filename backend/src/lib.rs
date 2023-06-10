use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

mod database;
mod handlers;
mod models;
mod routes;
mod schema;
pub mod server;

/// The application data.
pub struct AppState {
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
}

/// Creates an SSL connection manager.
fn ssl_builder() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file("rootCA-key.pem", SslFiletype::PEM)
        .expect("rootCA-key.pem must be in directory");
    builder
        .set_certificate_chain_file("rootCA.pem")
        .expect("rootCA.pem must be in directory");

    builder
}
