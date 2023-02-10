use actix_web::{App, HttpServer};

use crate::{database::database, routes::routes};

/// Starts the server.
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(database).configure(routes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
