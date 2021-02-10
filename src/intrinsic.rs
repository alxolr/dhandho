use crate::financial::pv;
use core::f32;

pub enum Multiplier {
    Outstanding,
    Standard,
}

#[derive(Debug, PartialEq)]
pub struct GrowthAssumption(pub u8, pub f32); // number of years, fcf growth rate

#[derive(Debug, PartialEq)]
pub struct GrowthAssumptionBuilder {
    pub assumptions: Vec<GrowthAssumption>,
}
impl GrowthAssumptionBuilder {
    pub fn new() -> Self {
        Self { assumptions: vec![] }
    }

    pub fn add(mut self, assumption: GrowthAssumption) -> GrowthAssumptionBuilder {
        self.assumptions.push(assumption);

        self
    }
}

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
            cash: Some(0.),
            rate: Some(0.15),     // default 15%
            multiplier: Some(10), // default 10x
            growth_assumptions: Some(
                GrowthAssumptionBuilder::new()
                    .add(GrowthAssumption(5, 0.05))
                    .add(GrowthAssumption(5, 0.05)),
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

    pub fn add_cash(mut self, cash: f32) -> IntrinsicBuilder {
        self.cash = Some(cash);

        self
    }

    pub fn compute(self) -> f32 {
        let mut result = 0.;
        let mut fcf = self.fcf.unwrap();
        let rate = self.rate.unwrap();
        let growth_assumptions = self.growth_assumptions.unwrap().assumptions;

        let mut year = 0;

        for growth in growth_assumptions.iter() {
            for _ in 1..=growth.0 {
                year += 1;
                fcf = fcf * (1. + growth.1);
                result += pv(rate, year, fcf); // discount the fcf
            }
        }

        let sale_price = pv(rate, year, fcf * self.multiplier.unwrap() as f32);
        result += sale_price + self.cash.unwrap();

        result.round()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intrisic_builder() {
        let expected = IntrinsicBuilder {
            cash: Some(0.),
            fcf: Some(15.),
            rate: Some(0.15),
            multiplier: Some(10),
            growth_assumptions: Some(
                GrowthAssumptionBuilder::new()
                    .add(GrowthAssumption(5, 0.05))
                    .add(GrowthAssumption(5, 0.05)),
            ),
        };

        let builded = IntrinsicBuilder::new()
            .add_fcf(15.)
            .add_rate(0.15)
            .add_multiplier(Multiplier::Standard);

        assert_eq!(expected, builded);
    }

    #[test]
    fn test_compute_on_bbby_example() {
        let intrisic = IntrinsicBuilder {
            fcf: Some(15.),
            rate: Some(0.15),
            cash: Some(40.),
            multiplier: Some(15),
            growth_assumptions: Some(
                GrowthAssumptionBuilder::new()
                    .add(GrowthAssumption(5, 0.05))
                    .add(GrowthAssumption(5, 0.05)),
            ),
        };

        assert_eq!(intrisic.compute(), 225.);
    }
}
