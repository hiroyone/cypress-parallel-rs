use std::process;

use cypress_parallel::cli;
use env_logger::Env;

#[tokio::main]
async fn main() {
    // Todo: Set the level to "info" after the initial development
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    cli::start().await.unwrap_or_else(|err| {
        eprintln!("Problem running the application: {}", err);
        process::exit(1);
    });
}
