use clap::Clap;

use super::port::Run;

#[derive(Clap, Debug)]
#[clap(about = "Computes the intrinsic value of an asset by providing the different parameters")]
pub struct IntrinsicCliImpl {
    #[clap(short, long, default_value = "0")]
    cash: String,
    #[clap(short, long, required = true)]
    free_cashflow: String,
    #[clap(short, long, default_value = "0.15")]
    rate: String,
    #[clap(short, long, default_value = "10")]
    multiplier: String,
    #[clap(short, long, multiple = true, required = true)]
    growth_assumptions: Vec<String>,
}

impl Run for IntrinsicCliImpl {
    fn run(self) {
        println!("{:?}", self);
    }
}
