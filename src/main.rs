mod cli;
mod core;
mod stats;
mod utils;

use clap::Clap;
use cli::intrinsic_cli_impl::IntrinsicCliImpl;
use cli::kelly_cli_impl::KellyCliImpl;
use cli::port::Run;
use dotenv::dotenv;

#[derive(Clap, Debug)]
#[clap(
    version = "1.0",
    author = "Alexandru Olaru. <alxolr@gmail.com>",
    rename_all = "kebab-case"
)]
enum Dhandho {
    Kelly(KellyCliImpl),
    Intrinsic(IntrinsicCliImpl),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    match Dhandho::parse() {
        Dhandho::Kelly(kelly) => kelly.run(),
        Dhandho::Intrinsic(intrinsic) => intrinsic.run(),
    }

    Ok(())
}
