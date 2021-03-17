use std::convert::TryFrom;

use clap::Clap;

use super::port::Run;
use crate::core::growth_assumption_builder::{GrowthAssumption, GrowthAssumptionBuilder};
use crate::core::intrinsic_builder::IntrinsicBuilder;
use crate::utils::money::Money;

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
                if s.len() == 3 {
                    GrowthAssumption(
                        s[0].parse::<u8>().unwrap(),
                        s[1].parse::<f32>().unwrap(),
                        Some(s[2].parse::<f32>().unwrap()),
                    )
                } else {
                    GrowthAssumption(
                        s[0].parse::<u8>().unwrap(),
                        s[1].parse::<f32>().unwrap(),
                        None,
                    )
                }
            })
            .map(|item| item.normalize())
            .flatten()
            .collect::<Vec<_>>();

        gab.assumptions = assumptions;

        let result = IntrinsicBuilder::new()
            .add_cash(Money::try_from(self.cash).unwrap())
            .add_fcf(Money::try_from(self.free_cashflow).unwrap())
            .add_growth_assumptions(gab)
            .add_rate(self.rate.parse::<f32>().unwrap_or(0.15))
            .add_multiplier(self.multiplier.parse::<u8>().unwrap_or(10))
            .execute();

        println!("{}", result);
    }
}
