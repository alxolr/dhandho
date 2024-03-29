#[derive(Debug, PartialEq, PartialOrd)]
pub struct KellyAssumption(pub f32, pub f32);

#[derive(Debug, PartialEq)]
pub struct KellyAssumptionBuilder {
    pub assumptions: Vec<KellyAssumption>,
}

impl KellyAssumptionBuilder {
    pub fn new() -> Self {
        KellyAssumptionBuilder {
            assumptions: vec![],
        }
    }

    pub fn set(mut self, assumptions: Vec<KellyAssumption>) -> KellyAssumptionBuilder {
        self.assumptions = assumptions;

        self
    }

    pub fn get_edge(&self) -> f32 {
        self.assumptions.iter().fold(0.0, |mut acc, it| {
            acc = acc + it.0 * it.1;

            acc
        })
    }

    pub fn compute(self) -> f32 {
        let max_wagger = self
            .assumptions
            .iter()
            .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
            .unwrap_or(self.assumptions.first().unwrap());

        let edge = self.get_edge();

        edge / max_wagger.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_kelly_assumptions() {
        let expected = KellyAssumptionBuilder {
            assumptions: vec![
                KellyAssumption(0.8, 21.0),
                KellyAssumption(0.1, 7.5),
                KellyAssumption(0.1, -1.0),
            ],
        };

        assert_eq!(
            KellyAssumptionBuilder::new().set(vec![
                KellyAssumption(0.8, 21.0),
                KellyAssumption(0.1, 7.5),
                KellyAssumption(0.1, -1.0),
            ]),
            expected
        );
    }

    #[test]
    fn test_should_return_the_number_for_the_allocation() {
        let assumptions = vec![
            KellyAssumption(0.8, 21.0),
            KellyAssumption(0.1, 7.5),
            KellyAssumption(0.1, -1.0),
        ];
        let kelly = KellyAssumptionBuilder::new().set(assumptions);

        assert_eq!(kelly.compute(), 0.8309524);
    }
}
