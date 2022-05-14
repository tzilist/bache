use bache::{config::Args, server};
use clap::Parser;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    let args = Args::parse();

    server::start(args).await
}
