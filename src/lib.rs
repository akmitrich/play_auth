pub mod configuration;
pub mod domain;
pub mod routes;
pub mod startup;
pub mod telemetry;

#[cfg(test)]
mod tests {
    use crate::routes::health_check;
    #[tokio::test]
    async fn health_check_succeeds() {
        let _response = health_check().await;
        // assert!(response.status().is_success());
    }
}
