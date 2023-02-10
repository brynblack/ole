use ole::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    server::run().await
}