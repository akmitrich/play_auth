#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    play_auth::run().await
}
