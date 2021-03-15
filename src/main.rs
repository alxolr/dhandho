mod intrinsic;
mod stats;
mod test_mock_example;
mod utils;
use clap::Clap;

use dotenv::dotenv;
use utils::kelly::Kelly;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Alexandru Olaru. <alxolr@gmail.com>", rename_all="kebab-case")]
enum Dhandho {
    Kelly(Kelly)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    match Dhandho::parse() {
        Dhandho::Kelly(kelly) => kelly.run()
    }

    Ok(())
}
