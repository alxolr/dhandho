mod financial;
mod growth_assumption;
mod intrinsic;
mod kelly;
mod money;
mod stats_provider;

use growth_assumption::{GrowthAssumption, GrowthAssumptionBuilder};
use intrinsic::{IntrinsicBuilder, Multiplier};
use kelly::{KellyAssumption, KellyAssumptionBuilder};
use money::Money;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
