use structopt::StructOpt;

use super::port::Run;
use crate::core::kelly_builder::{KellyAssumption, KellyAssumptionBuilder};
#[derive(StructOpt, Debug)]
#[structopt(about = "Maximaze the gains by providing different assumptions. Ex: -a 0.8,21.0")]
pub struct KellyCliImpl {
    #[structopt(short, long, multiple = true, required = true)]
    assumption: Vec<String>,

    #[structopt(short, long)]
    bankroll: Option<f32>,
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

        if self.bankroll.is_some() {
            let bankroll = self.bankroll.unwrap();
            println!("Bankroll: {}", bankroll);
            println!("Kelly: {}", result);
            println!("Amount to wagger: {}", result * bankroll);
        }
    }
}
