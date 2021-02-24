mod financial;
mod intrinsic_interactor;
mod kelly;
mod money;
mod stats_provider;
mod test_mock_example;

use stats_provider::yahoo::Yahoo;
use intrinsic_interactor::IntrinsicInteractor;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ticker = env::args().collect::<Vec<String>>().pop().unwrap().clone();
    let provider = Yahoo::new();

    let stats = IntrinsicInteractor::new(ticker, Box::new(provider))
        .execute()
        .await?;

    // .add_fcf(*fcf_first.unwrap())
    //     .add_cash(stats.total_cash.clone())
    //     .add_rate(0.15)
    //     .add_growth_assumptions(
    //         GrowthAssumptionBuilder::new()
    //             .add(GrowthAssumption(5, growth_rate, None))
    //             .add(GrowthAssumption(5, growth_rate, Some(-0.01))),
    //     )
    //     .compute();

    println!("{:?}", stats);

    Ok(())
}
