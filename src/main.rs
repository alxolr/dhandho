mod financial;
mod growth_assumption;
mod intrinsic;
mod kelly;
mod money;
mod stats_provider;
mod test_mock_example;

use growth_assumption::{GrowthAssumption, GrowthAssumptionBuilder};
use intrinsic::IntrinsicBuilder;
use stats_provider::{provider::Provider, yahoo::Yahoo};
use tokio::join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Yahoo::new("GPP".to_string());
    let (fcf, key_stats, analysis) = join!(
        provider.get_fcf(),
        provider.get_key_stats(),
        provider.get_analysis()
    );

    let growth_rate: f32 = analysis?;
    let stats = key_stats?;

    let intrinsic = IntrinsicBuilder::new()
        .add_fcf(fcf?.pop().unwrap())
        .add_cash(stats.total_cash.clone())
        .add_rate(0.15)
        .add_growth_assumptions(
            GrowthAssumptionBuilder::new()
                .add(GrowthAssumption(5, growth_rate, None))
                .add(GrowthAssumption(5, growth_rate, Some(-0.01))),
        )
        .compute();

    println!(
        "Let intrinsic {} vs market_cap {}",
        intrinsic, stats.mkt_cap
    );

    Ok(())
}
