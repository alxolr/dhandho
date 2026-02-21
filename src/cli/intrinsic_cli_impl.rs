use super::port::Run;
use crate::core::growth_assumption_builder::{GrowthAssumption, GrowthAssumptionBuilder};
use crate::core::intrinsic_builder::IntrinsicBuilder;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(about = "Computes the intrinsic value of an asset by providing different parameters")]
pub struct IntrinsicCliImpl {
    #[arg(
        short = 'f',
        long = "cashflow",
        required = true,
        help = "Current value of the asset"
    )]
    cashflow: f32,
    #[arg(short, long, default_value = "0.15", help = "Discount rate")]
    rate: f32,
    #[arg(
        short,
        long,
        required = true,
        help = "Growth assumptions in the format years,rate,growth_incr"
    )]
    growth_assumptions: Vec<String>,
    #[arg(short, long, help = "Optional cash amount")]
    cash: Option<f32>,
    #[arg(short, long, help = "Optional debt amount")]
    debt: Option<f32>,
    #[arg(short, long, help = "Optional probability of failure")]
    probability_of_failure: Option<f32>,
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
            .add_current_value(self.cashflow)
            .add_growth_assumptions(gab)
            .add_rate(self.rate)
            .add_cash(self.cash)
            .add_debt(self.debt)
            .add_probability_of_failure(self.probability_of_failure)
            .execute();
    }
}
