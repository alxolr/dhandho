pub fn pv(rate: f32, periods: i32, fv: f32) -> f32 {
    fv / (1. + rate).powi(periods)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_present_value_formula() {
        let rate = 0.10;
        let future_value = 10000.;
        let periods = 10;

        assert_eq!(pv(rate, periods, future_value,), 3855.432);
    }
}
