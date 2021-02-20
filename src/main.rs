mod financial;
mod growth_assumption;
mod intrinsic;
mod kelly;
mod money;
mod stats_provider;
mod stats_scrapers;
mod test_mock;

use stats_scrapers::{scraper::Scraper, yahoo::Yahoo};
use tokio::join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scraper = Yahoo::new("GPP".to_string());
    let (fcf, key_stats) = join!(scraper.get_fcf(), scraper.get_key_stats());

    println!("FCF = {:?}, KeyStats = {:?}", fcf, key_stats);

    Ok(())
}
