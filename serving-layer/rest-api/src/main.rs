mod handlers;
use actix_web::{get, web, App, HttpServer};
use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/route-counts", web::get().to(get_route_count))
    })
    .bind(("0.0.0.0", 8088))?
    .run()
    .await
}