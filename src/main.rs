use ole::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run().await
}
