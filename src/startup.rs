use actix_web::{dev, web, App, HttpServer};

use crate::routes::{health_check, subscribe};

pub fn run(listener: std::net::TcpListener, db_pool: sqlx::PgPool) -> std::io::Result<dev::Server> {
    let connection = web::Data::new(db_pool);
    Ok(HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run())
}
