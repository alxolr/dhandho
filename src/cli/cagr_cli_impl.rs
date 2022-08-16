use structopt::StructOpt;

use crate::utils::financial::cagr;

use super::port::Run;

#[derive(StructOpt, Debug)]
#[structopt(about = "Calculated the compounded anual growth rate")]
pub struct CagrCliImpl {
    final_value: f32,
    initial_value: f32,
    periods: i32,
}

impl Run for CagrCliImpl {
    fn run(self) {
        println!(
            "{}",
            cagr(self.final_value, self.initial_value, self.periods)
        )
    }
}
