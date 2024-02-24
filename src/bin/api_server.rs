#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = play_auth::telemetry::get_subscriber("play_auth", "info", std::io::stdout);
    play_auth::telemetry::init_subscriber(subscriber);

    let configuration =
        play_auth::configuration::get_configuration().expect("Failed to read configuration.");
    let connection_pool = sqlx::PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address)?;
    println!("Binded at {:?}", listener.local_addr()?);
    play_auth::startup::run(listener, connection_pool)?.await
}
