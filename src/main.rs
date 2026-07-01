use clap::Parser;
use mailghost::{run, Cli};

#[tokio::main]
async fn main() {
    if let Err(error) = run(Cli::parse()).await {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}
