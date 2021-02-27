use super::money::Money;

// Present Value
pub fn pv(rate: f32, periods: i32, fv: Money) -> Money {
    fv / (1. + rate).powi(periods)
}

// Compound Annual Growth Rate formula
pub fn cagr(ev: Money, bv: Money, periods: u32) -> f32 {
    ((ev / bv) as f32).powf(1.0 / periods as f32) - 1.0
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
}
