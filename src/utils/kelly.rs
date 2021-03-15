use clap::Clap;
#[derive(Clap, Debug)]
#[clap(
    about = "Formula used to maximaze the gains by providing different assumptions. Ex: -a 0.8,21.0"
)]
pub struct Kelly {
    #[clap(short, long, multiple = true, required = true)]
    assumption: Vec<String>,
}

impl Kelly {
    pub fn run(self) {
        let mut kelly_builder = KellyAssumptionBuilder::new();

        let assumptions = self
            .assumption
            .into_iter()
            .map(|it| {
                let numbers = it
                    .split(",")
                    .into_iter()
                    .map(|nb| nb.parse::<f32>().unwrap())
                    .collect::<Vec<_>>();

                KellyAssumption(*numbers.first().unwrap(), *numbers.last().unwrap())
            })
            .collect::<Vec<_>>();

        kelly_builder.assumptions = assumptions;

        let result = kelly_builder.compute();

        println!("{}", result);
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct KellyAssumption(f32, f32);

#[derive(Debug, PartialEq)]
struct KellyAssumptionBuilder {
    assumptions: Vec<KellyAssumption>,
}

impl KellyAssumptionBuilder {
    fn new() -> Self {
        KellyAssumptionBuilder {
            assumptions: vec![],
        }
    }

    fn add(mut self, assumption: KellyAssumption) -> KellyAssumptionBuilder {
        self.assumptions.push(assumption);

        self
    }

    fn compute(self) -> f32 {
        let max_wagger = self
            .assumptions
            .iter()
            .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
            .unwrap_or(self.assumptions.first().unwrap());

        let edge = self.assumptions.iter().fold(0.0, |mut acc, it| {
            acc = acc + it.0 * it.1;

            acc
        });

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
            KellyAssumptionBuilder::new()
                .add(KellyAssumption(0.8, 21.0))
                .add(KellyAssumption(0.1, 7.5))
                .add(KellyAssumption(0.1, -1.0)),
            expected
        );
    }

    #[test]
    fn test_should_return_the_number_for_the_allocation() {
        let kelly = KellyAssumptionBuilder::new()
            .add(KellyAssumption(0.8, 21.0))
            .add(KellyAssumption(0.1, 7.5))
            .add(KellyAssumption(0.1, -1.0));

        assert_eq!(kelly.compute(), 0.8309524);
    }
}
