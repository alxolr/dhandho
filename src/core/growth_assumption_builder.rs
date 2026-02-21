pub struct GrowthAssumption(pub u8, pub f32, pub Option<f32>);

impl GrowthAssumption {
    pub fn normalize(self) -> Vec<f32> {
        let mut result = vec![];
        let GrowthAssumption(range, mut rate, maybe_incr) = self;
        let incr = maybe_incr.unwrap_or(0.0);
        result.push((rate * 100.).round() / 100.0);

        for _ in 1..range {
            rate = ((rate + incr) * 100.0).round() / 100.0;
            result.push(rate);
        }

        result
    }
}
#[derive(PartialEq, Debug, Clone)]
pub struct GrowthAssumptionBuilder {
    pub assumptions: Vec<f32>,
}
impl GrowthAssumptionBuilder {
    pub fn new() -> Self {
        GrowthAssumptionBuilder {
            assumptions: vec![],
        }
    }

    pub fn add(mut self, assumption: GrowthAssumption) -> GrowthAssumptionBuilder {
        self.assumptions.extend_from_slice(&assumption.normalize());

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_one_assumption_per_year_with_specified_increment() {
        let expected = GrowthAssumptionBuilder {
            assumptions: vec![0.15, 0.14, 0.13, 0.12, 0.11, 0.10],
        };

        assert_eq!(
            GrowthAssumptionBuilder::new().add(GrowthAssumption(6, 0.15, Some(-0.01))),
            expected
        );
    }

    #[test]
    fn should_create_one_assumption_per_year_with_specified_plus_increment() {
        let expected = GrowthAssumptionBuilder {
            assumptions: vec![0.15, 0.16, 0.17, 0.18, 0.19, 0.20],
        };

        assert_eq!(
            GrowthAssumptionBuilder::new().add(GrowthAssumption(6, 0.15, Some(0.01))),
            expected
        );
    }
}
