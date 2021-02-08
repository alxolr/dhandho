mod intrisic;

use intrisic::IntrisicBuilder;

fn main() {
    println!("Hello, world!");
    let val = IntrisicBuilder::new()
        .free_cash_flow(15.)
        .discount_rate(0.15)
        .compute();

    println!("Intrisic Value {}", val);
}
