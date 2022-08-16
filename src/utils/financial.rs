/// Present Value
pub fn pv(rate: f32, periods: i32, fv: f32) -> f32 {
    fv / (1. + rate).powi(periods)
}

/// Compounded Annual Growth Rate
pub fn cagr(final_value: f32, initial_value:f32, periods: i32) -> f32 {
    (final_value/initial_value).powf(1.0f32 / periods as f32) - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_present_value_formula() {
        let rate = 0.10;
        let future_value = 10000.0;
        let periods = 10;

        assert_eq!(pv(rate, periods, future_value,), 3855.432);
    }

    #[test]
    fn test_cagr_value_formula() {
        let final_value = 95.0;
        let initial_value: f32 = 80.0;

        assert_eq!(cagr(final_value, initial_value, 1), 0.1875)
    }
}
