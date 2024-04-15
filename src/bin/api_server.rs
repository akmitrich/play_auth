use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = play_auth::telemetry::get_subscriber("play_auth", "info", std::io::stdout);
    play_auth::telemetry::init_subscriber(subscriber);

    let configuration =
        play_auth::configuration::get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = std::net::TcpListener::bind(address)?;
    println!("Binded at {:?}", listener.local_addr()?);
    play_auth::startup::run(listener, connection_pool)?.await
}
