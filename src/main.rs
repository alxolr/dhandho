mod intrinsic;
mod stats;
mod test_mock_example;
mod utils;

use dotenv::dotenv;
// use intrinsic::intrinsic_service::IntrinsicService;
use serde::Deserialize;
use stats::{repo::StatsRepo, stats_repo_impl::StatsRepoImpl};
// use std::{env, io, time::Duration};

#[derive(Debug, Deserialize)]
struct Record {
    ticker: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // let params = env::vars().into_iter().collect::<Vec<_>>();

    // println!("{:?}", params);
    let stats_repo = StatsRepoImpl::new();

    stats_repo.get_stats("GPP").await?;

    // let interactor = IntrinsicService::new(Box::new(StatsRepoImpl::new()));
    // let mut rdr = csv::ReaderBuilder::new()
    //     .has_headers(false)
    //     .from_reader(io::stdin());

    // let mut wtr = csv::Writer::from_writer(std::io::stdout());

    // for result in rdr.deserialize() {
    //     let record: Record = result?;
    //     let stats = interactor.execute(&record.ticker).await;
    //     if stats.is_ok() {
    //         wtr.serialize(stats?)?;
    //     }
    //     sleep(Duration::from_secs(5)).await;
    //     wtr.flush()?;
    // }

    // wtr.flush()?;

    Ok(())
}
