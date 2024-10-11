#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = std::net::TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port 8000");
    play_auth::run(listener)?.await
}
