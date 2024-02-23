#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration =
        play_auth::configuration::get_configuration().expect("Failed to read configuration.");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address)?;
    println!("Binded at {:?}", listener.local_addr()?);
    play_auth::startup::run(listener)?.await
}
