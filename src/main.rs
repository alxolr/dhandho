mod intrinsic;

use intrinsic::IntrinsicBuilder;

fn main() {
    println!("Hello, world!");
    let val = IntrinsicBuilder::new()
        .free_cash_flow(15.)
        .discount_rate(0.15)
        .compute();

    println!("Intrinsic Value {}", val);
}
