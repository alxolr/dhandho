use clap::Parser;

use super::port::Run;
use crate::core::kelly_builder::{KellyAssumption, KellyAssumptionBuilder};
#[derive(Parser, Debug)]
#[command(about = "Maximaze the gains by providing different assumptions. Ex: -a 0.8,21.0")]
pub struct KellyCliImpl {
    #[arg(short, long, required = true, help = "Assumptions in the format rate,amount")]
    assumption: Vec<String>,

    #[arg(short, long, help = "Optional bankroll amount")]
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

        let kelly_builder = KellyAssumptionBuilder::new().set(assumptions);
        let edge = kelly_builder.get_edge();
        let result = kelly_builder.compute();

        if self.bankroll.is_some() {
            let bankroll = self.bankroll.unwrap();

            if edge > 0.0 {
                println!("Bankroll: {}", bankroll);
                println!("Kelly: {}", result);
                println!("Amount to wagger: {}", result * bankroll);
                println!("Expected Value: {}", edge * (bankroll * result));
            } else {
                println!("Negative expected value. Don't wagger.")
            }
        } else {
            println!("Edge: {}", edge);
            println!("Kelly: {}", result);
            println!("Expected Value per unit: {}", edge);
        }
    }
}
