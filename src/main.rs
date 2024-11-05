#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_log::LogTracer::init().expect("Failed to set logger");
    let subscriber =
        play_auth::telemetry::get_subscriber("play_auth".into(), "info".into(), std::io::stdout);
    play_auth::telemetry::init_subscriber(subscriber);
    let configuration =
        play_auth::configuration::get_configuration().expect("Failed to read configuration.");
    let connection = sqlx::PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address)?;
    play_auth::startup::run(listener, connection)?.await
}
