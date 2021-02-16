mod financial;
mod growth_assumption;
mod intrinsic;
mod kelly;

use growth_assumption::{GrowthAssumption, GrowthAssumptionBuilder};
use intrinsic::{IntrinsicBuilder, Multiplier};

fn main() {
    println!("Hello, world!");
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

    println!("Intrinsic Value {}", val);
}
