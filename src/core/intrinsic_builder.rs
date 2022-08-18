use super::growth_assumption_builder::{GrowthAssumption, GrowthAssumptionBuilder};
use crate::utils::financial::pv;
use core::f32;

#[derive(PartialEq, Debug)]
pub struct IntrinsicBuilder {
    current_value: Option<f32>,
    // expected rate of return for the investment
    rate: Option<f32>,
    // usually it's 10 or 15 for premium bussinesses
    multiplier: Option<u8>,
    // some growth assumptions
    growth_assumptions: Option<GrowthAssumptionBuilder>,
}

impl IntrinsicBuilder {
    pub fn new() -> IntrinsicBuilder {
        IntrinsicBuilder {
            current_value: None,
            rate: Some(0.15),     // default 15%
            multiplier: Some(15), // default 10x
            growth_assumptions: Some(
                GrowthAssumptionBuilder::new()
                    .add(GrowthAssumption(5, 0.05, None))
                    .add(GrowthAssumption(5, 0.05, None)),
            ), // default 5 % of fcf growth
        }
    }

    pub fn add_current_value(mut self, current_value: f32) -> IntrinsicBuilder {
        self.current_value = Some(current_value);

        self
    }

    pub fn add_rate(mut self, rate: f32) -> IntrinsicBuilder {
        self.rate = Some(rate);

        self
    }

    pub fn add_multiplier(mut self, multiplier: u8) -> IntrinsicBuilder {
        self.multiplier = Some(multiplier);

        self
    }

    pub fn add_growth_assumptions(mut self, growths: GrowthAssumptionBuilder) -> IntrinsicBuilder {
        self.growth_assumptions = Some(growths);

        self
    }

    pub fn execute(self) -> f32 {
        let mut result = 0.0;
        let mut current_value = self.current_value.unwrap();
        let multiple = self.multiplier.unwrap_or(15);
        let rate = self.rate.unwrap();
        let growth_assumptions = self.growth_assumptions.unwrap().assumptions;

        println!(
            "\nRate: {},\nTerminal Multiple: {},\nGrowth assumptions: {:?}",
            rate, multiple, growth_assumptions
        );

        println!("\n{0: <4} | {1: <10} | {2: <10}", "year", "fv", "pv");
        println!("{}", "-".repeat(30));

        let mut year = 0;
        println!("{0: <4} | {1: <10} | {2: <10}", year, 0, current_value);

        for assumption_rate in growth_assumptions.iter() {
            year += 1;
            current_value = current_value * (1. + assumption_rate);
            let pv = pv(rate, year, current_value);

            println!("{0: <4} | {1: <10} | {2: <10}", year, current_value, pv);
            result += pv;
        }


        let termial_value = current_value * self.multiplier.unwrap() as f32;
        let sale_price = pv(rate, year, termial_value);

        println!(
            "{0: <4} | {1: <10} | {2: <10}",
            "TV", termial_value, sale_price
        );

        println!("{}", "-".repeat(30));
        result += sale_price;

        println!(
            "{0: <17} | {1: <10}\n",
            "NPV", result
        );


        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intrisic_builder() {
        let expected = IntrinsicBuilder {
            current_value: Some(15.0),
            rate: Some(0.15),
            multiplier: Some(10),
            growth_assumptions: Some(
                GrowthAssumptionBuilder::new().add(GrowthAssumption(10, 0.05, None)),
            ),
        };

        let builded = IntrinsicBuilder::new()
            .add_current_value(15.0)
            .add_rate(0.15)
            .add_growth_assumptions(
                GrowthAssumptionBuilder::new().add(GrowthAssumption(10, 0.05, None)),
            )
            .add_multiplier(10);

        assert_eq!(expected, builded);
    }

    #[test]
    fn test_compute_on_simple_example() {
        let intrisic = IntrinsicBuilder::new()
            .add_current_value(15.0)
            .add_rate(0.15)
            .add_multiplier(15)
            .add_growth_assumptions(
                GrowthAssumptionBuilder::new().add(GrowthAssumption(10, 0.05, None)),
            );

        assert_eq!(intrisic.execute(), 184.67798);
    }
}
