#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration =
        play_auth::configuration::get_configuration().expect("Failed to read configuration.");
    let connection = sqlx::PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address)?;
    play_auth::startup::run(listener, connection)?.await
}
