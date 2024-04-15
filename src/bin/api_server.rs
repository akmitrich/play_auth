use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = play_auth::telemetry::get_subscriber("play_auth", "info", std::io::stdout);
    play_auth::telemetry::init_subscriber(subscriber);

    let configuration =
        play_auth::configuration::get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        sqlx::PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = std::net::TcpListener::bind(address)?;
    println!("Binded at {:?}", listener.local_addr()?);
    play_auth::startup::run(listener, connection_pool)?.await
}
