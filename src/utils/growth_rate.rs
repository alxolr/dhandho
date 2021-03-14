use super::{
    financial::{cagr, avg},
    money::Money,
};

pub fn get_growth_rate(items: Vec<Money>) -> f32 {
    let mut current_size = items.len() - 1;

    if current_size >= 2 {
        let mut growths: Vec<f32> = vec![];

        loop {
            if current_size >= 2 {
                let results = items
                    .windows(current_size)
                    .map(|n| compute_cagr(n))
                    .collect::<Vec<f32>>();

                // destructure and concat 2 slices
                growths = [&growths[..], &results[..]].concat();
            } else {
                break;
            }
            current_size = current_size - 1;
        }

        avg(&mut growths)
    } else {
        0.05
    }
}

fn compute_cagr(items: &[Money]) -> f32 {
    (cagr(
        *items.first().unwrap(),
        *items.last().unwrap(),
        items.len() - 1,
    ) * 100.0)
        .round()
        / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growth_rate_scenarios() {
        let scenarios = vec![
            (vec![Money(1200), Money(1100), Money(1000), Money(900)], 0.102),
        ];

        for scenario in scenarios {
            let (items, expected_rate) = scenario;
            assert_eq!(get_growth_rate(items), expected_rate);
        }
    }
}
