mod financial;
mod intrinsic_interactor;
mod kelly;
mod money;
mod stats_provider;
mod test_mock_example;

use intrinsic_interactor::IntrinsicInteractor;
use serde::Deserialize;
use stats_provider::yahoo::Yahoo;
use std::io;

#[derive(Debug, Deserialize)]
struct Record {
    ticker: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let ticker = env::args().collect::<Vec<String>>().pop().unwrap().clone();
    let provider = Yahoo::new();
    let interactor = intrinsic_interactor::IntrinsicInteractor::new(Box::new(provider));

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());

    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    for result in rdr.deserialize() {
        let record: Record = result?;
        let stats = interactor.execute(&record.ticker).await;
        if stats.is_ok() {
            wtr.serialize(stats?)?;
        }
        wtr.flush()?;
    }

    wtr.flush()?;

    Ok(())
}
