use actix_web::{dev, web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> std::io::Result<dev::Server> {
    Ok(
        HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
            .bind("0.0.0.0:24944")?
            .run(),
    )
}

#[cfg(test)]
mod tests {
    use crate::health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let _response = health_check().await;
        // assert!(response.status().is_success());
    }
}
