use super::port::Run;
use crate::core::growth_assumption_builder::{GrowthAssumption, GrowthAssumptionBuilder};
use crate::core::intrinsic_builder::IntrinsicBuilder;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Computes the intrinsic value of an asset by providing the different parameters"
)]
pub struct IntrinsicCliImpl {
    #[structopt(short, long, required = true)]
    current_value: f32,
    #[structopt(short, long, default_value = "0.15")]
    rate: f32,
    #[structopt(short, long, default_value = "15")]
    multiplier: u8,
    #[structopt(short, long, multiple = true, required = true)]
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

        IntrinsicBuilder::new()
            .add_current_value(self.current_value)
            .add_growth_assumptions(gab)
            .add_rate(self.rate)
            .add_multiplier(self.multiplier)
            .execute();
    }
}
