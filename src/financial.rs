use crate::money::Money;

pub fn pv(rate: f32, periods: i32, fv: Money) -> Money {
    fv / (1. + rate).powi(periods)
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
}
