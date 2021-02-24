use crate::{financial::pv, money::Money};
use core::f32;

use super::growth_assumption::{GrowthAssumption, GrowthAssumptionBuilder};

pub enum Multiplier {
    Outstanding,
    Standard,
}

#[derive(PartialEq, Debug)]
pub struct IntrinsicBuilder {
    cash: Option<Money>,
    // free cash flow
    fcf: Option<Money>,
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
            cash: Some(Money(0)),
            rate: Some(0.15),     // default 15%
            multiplier: Some(10), // default 10x
            growth_assumptions: Some(
                GrowthAssumptionBuilder::new()
                    .add(GrowthAssumption(5, 0.05, None))
                    .add(GrowthAssumption(5, 0.05, None)),
            ), // default 5 % of fcf growth
        }
    }

    pub fn add_fcf(mut self, cash: Money) -> IntrinsicBuilder {
        self.fcf = Some(cash);

        self
    }

    pub fn add_rate(mut self, rate: f32) -> IntrinsicBuilder {
        self.rate = Some(rate);

        self
    }

    pub fn add_multiplier(mut self, multiplier: Multiplier) -> IntrinsicBuilder {
        match multiplier {
            Multiplier::Outstanding => self.multiplier = Some(15),
            Multiplier::Standard => self.multiplier = Some(10),
        }

        self
    }

    pub fn add_growth_assumptions(mut self, growths: GrowthAssumptionBuilder) -> IntrinsicBuilder {
        self.growth_assumptions = Some(growths);

        self
    }

    pub fn add_cash(mut self, cash: Money) -> IntrinsicBuilder {
        self.cash = Some(cash);

        self
    }

    pub fn execute(self) -> Money {
        let mut result = Money(0);
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
            cash: Some(Money(10)),
            fcf: Some(Money(15)),
            rate: Some(0.15),
            multiplier: Some(10),
            growth_assumptions: Some(
                GrowthAssumptionBuilder::new().add(GrowthAssumption(10, 0.05, None)),
            ),
        };

        let builded = IntrinsicBuilder::new()
            .add_fcf(Money(15))
            .add_cash(Money(10))
            .add_rate(0.15)
            .add_growth_assumptions(
                GrowthAssumptionBuilder::new().add(GrowthAssumption(10, 0.05, None)),
            )
            .add_multiplier(Multiplier::Standard);

        assert_eq!(expected, builded);
    }

    #[test]
    fn test_compute_on_simple_example() {
        let intrisic = IntrinsicBuilder::new()
            .add_cash(Money(40))
            .add_fcf(Money(15))
            .add_rate(0.15)
            .add_multiplier(Multiplier::Outstanding)
            .add_growth_assumptions(
                GrowthAssumptionBuilder::new().add(GrowthAssumption(10, 0.05, None)),
            );

        assert_eq!(intrisic.execute(), Money(231));
    }
}
