#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Binded at 0.0.0.0:24944");
    play_auth::run()?.await
}
