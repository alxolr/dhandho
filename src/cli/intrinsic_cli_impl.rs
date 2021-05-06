use clap::Clap;

use super::port::Run;
use crate::core::growth_assumption_builder::{GrowthAssumption, GrowthAssumptionBuilder};
use crate::core::intrinsic_builder::IntrinsicBuilder;

#[derive(Clap, Debug)]
#[clap(about = "Computes the intrinsic value of an asset by providing the different parameters")]
pub struct IntrinsicCliImpl {
    #[clap(short, long, default_value = "0")]
    cash: String,
    #[clap(short, long, required = true)]
    free_cashflow: String,
    #[clap(short, long, default_value = "0.15")]
    rate: String,
    #[clap(short, long, default_value = "10")]
    multiplier: String,
    #[clap(short, long, multiple = true, required = true)]
    growth_assumptions: Vec<String>,
}

impl Run for IntrinsicCliImpl {
    fn run(self) {
        let mut gab = GrowthAssumptionBuilder::new();

        let assumptions = self
            .growth_assumptions
            .into_iter()
            .map(|item| {
                let s = item.split(",").collect::<Vec<_>>();
                let years = s.get(0).unwrap().parse::<f32>().unwrap();
                let rate = s.get(1).unwrap().parse::<f32>().unwrap();
                let growth_incr = s.get(2).unwrap_or(&"0.0").parse::<f32>().unwrap();
                GrowthAssumption(years as u8, rate, Some(growth_incr))
            })
            .map(|item| item.normalize())
            .flatten()
            .collect::<Vec<_>>();

        gab.assumptions = assumptions;

        let result = IntrinsicBuilder::new()
            .add_cash(self.cash.parse::<f32>().unwrap())
            .add_fcf(self.free_cashflow.parse::<f32>().unwrap())
            .add_growth_assumptions(gab)
            .add_rate(self.rate.parse::<f32>().unwrap_or(0.15))
            .add_multiplier(self.multiplier.parse::<u8>().unwrap_or(10))
            .execute();

        println!("{}", result);
    }
}
