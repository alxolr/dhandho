use core::f32;

pub enum Multiplier {
    Outstanding,
    Standard,
}

#[derive(PartialEq, Debug)]
pub struct IntrisicBuilder {
    // how much cash a business is making per year usually in millions
    free_cash_flow: Option<f32>,
    // expected rate of return for the investment
    discount_rate: Option<f32>,
    // usually it's 10 or 15 for premium bussinesses
    premium_multiplier: Option<u8>,
    // estimated percentages for growth 1-5 years
    year_one_five_growth: Option<f32>,
    // estiamted percentages for growth 6-10 years
    year_six_ten_growth: Option<f32>,
}

impl IntrisicBuilder {
    pub fn new() -> IntrisicBuilder {
        IntrisicBuilder {
            free_cash_flow: None,
            discount_rate: Some(0.15),        // default 15%
            premium_multiplier: Some(10),     // default 10x
            year_six_ten_growth: Some(0.05),  // default 5%
            year_one_five_growth: Some(0.05), // default 5%
        }
    }

    pub fn free_cash_flow(mut self, cash: f32) -> IntrisicBuilder {
        self.free_cash_flow = Some(cash);

        self
    }

    pub fn discount_rate(mut self, rate: f32) -> IntrisicBuilder {
        self.discount_rate = Some(rate);

        self
    }

    pub fn premium_multiplier(mut self, multiplier: Multiplier) -> IntrisicBuilder {
        match multiplier {
            Multiplier::Outstanding => self.premium_multiplier = Some(15),
            Multiplier::Standard => self.premium_multiplier = Some(10),
        }

        self
    }

    pub fn year_one_five(mut self, growth: f32) -> IntrisicBuilder {
        self.year_one_five_growth = Some(growth);

        self
    }

    pub fn year_six_ten(mut self, growth: f32) -> IntrisicBuilder {
        self.year_six_ten_growth = Some(growth);

        self
    }

    pub fn compute(self) -> f32 {
        let mut total = 0.;
        let free_cash_flow = self.free_cash_flow.unwrap();
        let discount_rate = self.discount_rate.unwrap();
        let year_one_five_growth = self.year_one_five_growth.unwrap();
        let year_six_ten_growth = self.year_six_ten_growth.unwrap();

        for year in 1..=10 {
            let discount_divisor = (1. + discount_rate).powi(year);
            if year == 1 {
                total += free_cash_flow / discount_divisor;
            } else if year < 6 && year > 1 {
                // apply five years growth and discount
                let updated_cashflow = free_cash_flow * (1. + year_one_five_growth).powi(year - 1);
                total += updated_cashflow / discount_divisor;
            } else if year < 10 && year >= 6 {
                // apply six year growth and discount
                let updated_cashflow = free_cash_flow * (1. + year_six_ten_growth).powi(year - 1);
                total += updated_cashflow / discount_divisor;
            } else if year == 10 {
                // apply six year growth
                let updated_cashflow = free_cash_flow * (1. + year_six_ten_growth).powi(year - 1);
                let business_sell_price =
                    updated_cashflow * self.premium_multiplier.unwrap() as f32;
                // apply discount
                total += ((updated_cashflow + business_sell_price) / discount_divisor).floor();
            }
        }

        total.round()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intrisic_builder() {
        let expected = IntrisicBuilder {
            free_cash_flow: Some(15.),
            discount_rate: Some(0.15),
            premium_multiplier: Some(10),
            year_one_five_growth: Some(0.05),
            year_six_ten_growth: Some(0.05),
        };

        let builded = IntrisicBuilder::new()
            .free_cash_flow(15.)
            .discount_rate(0.15)
            .year_one_five(0.05)
            .year_six_ten(0.05)
            .premium_multiplier(Multiplier::Standard);

        assert_eq!(expected, builded);
    }

    #[test]
    fn test_compute_on_egy_example() {
        let intrisic = IntrisicBuilder {
            free_cash_flow: Some(15.),
            discount_rate: Some(0.15),
            premium_multiplier: Some(15),
            year_one_five_growth: Some(0.05),
            year_six_ten_growth: Some(0.05),
        };

        assert_eq!(intrisic.compute(), 176.);
    }
}
