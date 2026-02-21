mod cli;
mod core;
mod utils;

use clap::Parser;
use cli::cagr_cli_impl::CagrCliImpl;
use cli::intrinsic_cli_impl::IntrinsicCliImpl;
use cli::kelly_cli_impl::KellyCliImpl;
use cli::port::Run;

#[derive(Parser, Debug)]
#[command(
    version = "0.3.0",
    author = "Alexandru Olaru. <alxolr@gmail.com>",
    rename_all = "kebab-case"
)]
enum Dhandho {
    Kelly(KellyCliImpl),
    Intrinsic(IntrinsicCliImpl),
    Cagr(CagrCliImpl),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Dhandho::parse() {
        Dhandho::Kelly(kelly) => kelly.run(),
        Dhandho::Intrinsic(intrinsic) => intrinsic.run(),
        Dhandho::Cagr(cagr) => cagr.run(),
    }

    Ok(())
}
