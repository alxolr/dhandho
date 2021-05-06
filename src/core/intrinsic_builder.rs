use super::growth_assumption_builder::{GrowthAssumption, GrowthAssumptionBuilder};
use crate::utils::financial::pv;
use core::f32;

#[derive(PartialEq, Debug)]
pub struct IntrinsicBuilder {
    cash: Option<f32>,
    // free cash flow
    fcf: Option<f32>,
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
            fcf: None,
            cash: Some(0.0),
            rate: Some(0.15),     // default 15%
            multiplier: Some(10), // default 10x
            growth_assumptions: Some(
                GrowthAssumptionBuilder::new()
                    .add(GrowthAssumption(5, 0.05, None))
                    .add(GrowthAssumption(5, 0.05, None)),
            ), // default 5 % of fcf growth
        }
    }

    pub fn add_fcf(mut self, cash: f32) -> IntrinsicBuilder {
        self.fcf = Some(cash);

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

    pub fn add_cash(mut self, cash: f32) -> IntrinsicBuilder {
        self.cash = Some(cash);

        self
    }

    pub fn execute(self) -> f32 {
        let mut result = 0.0;
        let mut fcf = self.fcf.unwrap();
        let rate = self.rate.unwrap();
        let growth_assumptions = self.growth_assumptions.unwrap().assumptions;

        let mut year = 0;

        for assumption_rate in growth_assumptions.iter() {
            year += 1;
            fcf = fcf * (1. + assumption_rate);
            result += pv(rate, year, fcf);
        }

        let sale_price = pv(rate, year, fcf * self.multiplier.unwrap() as f32);
        result += sale_price + self.cash.unwrap();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intrisic_builder() {
        let expected = IntrinsicBuilder {
            cash: Some(10.0),
            fcf: Some(15.0),
            rate: Some(0.15),
            multiplier: Some(10),
            growth_assumptions: Some(
                GrowthAssumptionBuilder::new().add(GrowthAssumption(10, 0.05, None)),
            ),
        };

        let builded = IntrinsicBuilder::new()
            .add_fcf(15.0)
            .add_cash(10.0)
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
            .add_cash(40.0)
            .add_fcf(15.0)
            .add_rate(0.15)
            .add_multiplier(15)
            .add_growth_assumptions(
                GrowthAssumptionBuilder::new().add(GrowthAssumption(10, 0.05, None)),
            );

        assert_eq!(intrisic.execute(), 224.67798);
    }
}
