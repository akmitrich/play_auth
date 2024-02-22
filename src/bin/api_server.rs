use actix_web::{web, App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Binded at 0.0.0.0:24944");
    HttpServer::new(|| App::new().route("/health_check", web::get().to(play_auth::health_check)))
        .bind("0.0.0.0:24944")?
        .run()
        .await
}
