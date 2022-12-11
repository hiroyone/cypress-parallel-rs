use std::process;

use cypress_parallel::cli;


#[tokio::main]
async fn main() {
    cli::start().await.unwrap_or_else(|err| {
        eprintln!("Problem running the application: {}", err);
        process::exit(1);
    });
}