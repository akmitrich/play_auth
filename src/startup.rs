use actix_web::{dev, web, App, HttpServer};

use crate::routes::{health_check, subscribe};

pub fn run(listener: std::net::TcpListener) -> std::io::Result<dev::Server> {
    Ok(HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run())
}
