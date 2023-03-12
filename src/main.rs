mod cli;
mod core;
mod utils;

use cli::cagr_cli_impl::CagrCliImpl;
use cli::intrinsic_cli_impl::IntrinsicCliImpl;
use cli::kelly_cli_impl::KellyCliImpl;
use cli::port::Run;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    version = "0.2.1",
    author = "Alexandru Olaru. <alxolr@gmail.com>",
    rename_all = "kebab-case"
)]
enum Dhandho {
    Kelly(KellyCliImpl),
    Intrinsic(IntrinsicCliImpl),
    Cagr(CagrCliImpl),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Dhandho::from_args() {
        Dhandho::Kelly(kelly) => kelly.run(),
        Dhandho::Intrinsic(intrinsic) => intrinsic.run(),
        Dhandho::Cagr(cagr) => cagr.run(),
    }

    Ok(())
}
