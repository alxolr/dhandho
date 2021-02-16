mod financial;
mod growth_assumption;
mod intrinsic;
mod kelly;

use growth_assumption::{GrowthAssumption, GrowthAssumptionBuilder};
use intrinsic::{IntrinsicBuilder, Multiplier};
use kelly::{KellyAssumption, KellyAssumptionBuilder};

fn main() {
    let val = IntrinsicBuilder::new()
        .add_fcf(3678.)
        .add_cash(1331.)
        .add_rate(0.15)
        .add_multiplier(Multiplier::Outstanding)
        .add_growth_assumptions(
            GrowthAssumptionBuilder::new()
                .add(GrowthAssumption(5, 0.15, None))
                .add(GrowthAssumption(5, 0.10, None)),
        )
        .compute();

    let kelly = KellyAssumptionBuilder::new()
        .add(KellyAssumption(0.45, 2.2))
        .add(KellyAssumption(0.5, 0.1))
        .add(KellyAssumption(0.5, -1.0))
        .compute();

    println!("Intrinsic Value = {}", val);
    println!("Kelly allocation = {}", kelly);
}
