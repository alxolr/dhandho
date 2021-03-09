use core::num;

use super::money::Money;

// Present Value
pub fn pv(rate: f32, periods: i32, fv: Money) -> Money {
    fv / (1. + rate).powi(periods)
}

// Compound Annual Growth Rate formula
pub fn cagr(ev: Money, bv: Money, periods: usize) -> f32 {
    ((ev / bv) as f32).powf(1.0 / periods as f32) - 1.0
}

pub fn median(numbers: &mut [f32]) -> f32 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = numbers.len() / 2;
    numbers[mid]
}

pub fn avg(numbers: &mut [f32]) -> f32 {
    let len = numbers.len();
    numbers.iter().sum::<f32>() / len as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_present_value_formula() {
        let rate = 0.10;
        let future_value = Money(10000);
        let periods = 10;

        assert_eq!(pv(rate, periods, future_value,), Money(3855));
    }

    #[test]
    fn test_compund_annual_growth_rate_formula() {
        let ev = Money(15349);
        let bv = Money(10000);
        let periods = 5;

        assert_eq!(cagr(ev, bv, periods), 0.08947182)
    }

    #[test]
    fn test_median() {
        let mut numbers = vec![0.1, 0.09, 0.08];
        assert_eq!(median(&mut numbers), 0.09);
    }

    #[test]
    fn test_average() {
        let mut numbers = vec![0.1, 0.09, 0.01, 0.04];
        assert_eq!(avg(&mut numbers), 0.060000002);
    }
}
