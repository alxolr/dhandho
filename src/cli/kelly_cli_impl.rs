use clap::Clap;

use super::port::Run;
use crate::core::kelly_builder::{KellyAssumption, KellyAssumptionBuilder};
#[derive(Clap, Debug)]
#[clap(about = "Maximaze the gains by providing different assumptions. Ex: -a 0.8,21.0")]
pub struct KellyCliImpl {
    #[clap(short, long, multiple = true, required = true)]
    assumption: Vec<String>,
}

impl Run for KellyCliImpl {
    fn run(self) {
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

        let result = KellyAssumptionBuilder::new().set(assumptions).compute();

        println!("{}", result);
    }
}
