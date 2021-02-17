mod financial;
mod growth_assumption;
mod intrinsic;
mod kelly;

use growth_assumption::{GrowthAssumption, GrowthAssumptionBuilder};
use intrinsic::{IntrinsicBuilder, Multiplier};
use kelly::{KellyAssumption, KellyAssumptionBuilder};
use select::{document, predicate};
use tendril::StrTendril;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let val = IntrinsicBuilder::new()
        .add_fcf(3678.)
        .add_cash(1331.)
        .add_rate(0.15)
        .add_multiplier(Multiplier::Outstanding)
        .add_growth_assumptions(
            GrowthAssumptionBuilder::new()
                .add(GrowthAssumption(5, 0.15, None))
                .add(GrowthAssumption(5, 0.10, None)),
        )
        .compute();

    let kelly = KellyAssumptionBuilder::new()
        .add(KellyAssumption(0.45, 2.2))
        .add(KellyAssumption(0.5, 0.1))
        .add(KellyAssumption(0.5, -1.0))
        .compute();

    println!("Intrinsic Value = {}", val);
    println!("Kelly allocation = {}", kelly);

    let ticker = "GPP";
    let body = reqwest::blocking::get(&format!(
        "https://finance.yahoo.com/quote/{}/cash-flow?p={}",
        ticker, ticker
    ))?
    .text()?;

    let document = document::Document::from(StrTendril::from(body));
    let fcf_nodes = document
        .find(predicate::Name("span"))
        .filter(|i| i.text().contains("Free"))
        .next()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let node = fcf_nodes.descendants();
    let r = node
        .filter(|n| n.attr("data-test").unwrap_or("false") == "fin-col")
        .map(|n| n.text())
        .collect::<Vec<_>>();

    println!("{:?}", r);

    let values = r
        .iter()
        .filter(|n| n.len() > 1)
        .map(|n| n.trim().replace(",", "").parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!("{:?}", values);
    // for node in fcf_nodes.descendants(){

    // println!("span: {:?}", spans.as_text());
    // }

    // let values = spans
    //     .find(predicate::Attr("data-test", "fin-col"))
    //     .map(|n| n.find(predicate::Name("span")).next());
    // let root = document.root_element();
    // let selector = r#"//span[contains(text(), "Free")]"#;

    // // let val = document.select(&selector);

    // root.each_node(selector, |node| {
    //     println!("{:?}", node.value());
    // });

    Ok(())
}
