// Binary entry point - just calls the library function
use midi_backend::start_server;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    start_server().await
}
