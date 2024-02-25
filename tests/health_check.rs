use secrecy::ExposeSecret;
use sqlx::Connection;

pub struct TestApp {
    pub address: String,
    pub db_pool: sqlx::PgPool,
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to send health check request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let configuration =
        play_auth::configuration::get_configuration().expect("Failed to get app configuration.");
    let connection_string = configuration.database.connection_string();
    let mut connection = sqlx::PgConnection::connect(&connection_string.expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send subscriptions request");
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!("ursula_le_guin@gmail.com", saved.email);
    assert_eq!("le guin", saved.name);
}

#[tokio::test]
async fn subscribe_returns_400_when_data_are_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send subscriptions request");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when payload was {:?}",
            error_message
        );
    }
}

static TRACING: once_cell::sync::Lazy<()> = once_cell::sync::Lazy::new(|| {
    let default_filter_level = "debug";
    let subscriber_name = "test";
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = play_auth::telemetry::get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::stdout,
        );
        play_auth::telemetry::init_subscriber(subscriber);
    } else {
        let subscriber = play_auth::telemetry::get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::sink,
        );
        play_auth::telemetry::init_subscriber(subscriber);
    }
});

async fn spawn_app() -> TestApp {
    once_cell::sync::Lazy::force(&TRACING);

    let listener = std::net::TcpListener::bind("0.0.0.0:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let mut configuration =
        play_auth::configuration::get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();
    let connection_pool = configuration.database.configure_test_database().await;
    let server = play_auth::startup::run(listener, connection_pool.clone())
        .expect("Failed to spawn application");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}
