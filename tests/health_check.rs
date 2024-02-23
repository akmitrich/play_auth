#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:24944/health_check")
        .send()
        .await
        .expect("Failed to send health check request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = play_auth::run().expect("Failed to spawn application");
    let _ = tokio::spawn(server);
}