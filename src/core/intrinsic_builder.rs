use super::growth_assumption_builder::{GrowthAssumption, GrowthAssumptionBuilder};
use crate::utils::financial::pv;
use comfy_table::{presets, Cell, ColumnConstraint, Table, Width};
use core::f32;

#[derive(PartialEq, Debug)]
pub struct IntrinsicBuilder {
    current_value: Option<f32>,
    // expected rate of return for the investment
    rate: Option<f32>,
    // some growth assumptions
    growth_assumptions: GrowthAssumptionBuilder,
    // optional cash amount
    cash: Option<f32>,
    // optional debt amount
    debt: Option<f32>,
    // optional probability of failure
    probability_of_failure: Option<f32>,
    // optional shares outstanding
    shares_outstanding: Option<u32>,
}

impl IntrinsicBuilder {
    pub fn new() -> IntrinsicBuilder {
        IntrinsicBuilder {
            current_value: None,
            rate: Some(0.15), // default 15%
            growth_assumptions: GrowthAssumptionBuilder::new()
                .add(GrowthAssumption(5, 0.05, None))
                .add(GrowthAssumption(5, 0.05, None)), // default 5 % of fcf growth
            cash: None,
            debt: None,
            probability_of_failure: None,
            shares_outstanding: None,
        }
    }

    pub fn add_current_value(mut self, current_value: f32) -> IntrinsicBuilder {
        self.current_value = Some(current_value);

        self
    }

    pub fn add_rate(mut self, rate: f32) -> IntrinsicBuilder {
        self.rate = Some(rate);

        self
    }

    pub fn add_growth_assumptions(mut self, growths: GrowthAssumptionBuilder) -> IntrinsicBuilder {
        self.growth_assumptions = growths;

        self
    }

    pub fn add_cash(mut self, cash: Option<f32>) -> IntrinsicBuilder {
        self.cash = cash;

        self
    }

    pub fn add_debt(mut self, debt: Option<f32>) -> IntrinsicBuilder {
        self.debt = debt;

        self
    }

    pub fn add_probability_of_failure(
        mut self,
        probability_of_failure: Option<f32>,
    ) -> IntrinsicBuilder {
        self.probability_of_failure = probability_of_failure;

        self
    }

    pub fn add_shares_outstanding(mut self, shares_outstanding: Option<u32>) -> IntrinsicBuilder {
        self.shares_outstanding = shares_outstanding;

        self
    }

    pub fn execute(self) -> f32 {
        let mut result = 0.0;
        let mut current_value = self.current_value.unwrap();
        let rate = self.rate.unwrap();
        let growth_assumptions = &self.growth_assumptions.assumptions;

        // Table 1: ASSUMPTIONS
        let mut assumptions_table = Table::new();
        assumptions_table.load_preset(presets::UTF8_BORDERS_ONLY);
        assumptions_table.set_header(vec!["Assumptions", "Value"]);
        assumptions_table.set_constraints(vec![
            ColumnConstraint::Absolute(Width::Fixed(16)),
            ColumnConstraint::Absolute(Width::Fixed(16)),
        ]);

        assumptions_table.add_row(vec![
            Cell::new("Initial Value (FCF)"),
            Cell::new(format!("${:.2}", current_value)),
        ]);
        assumptions_table.add_row(vec![
            Cell::new("Discount Rate"),
            Cell::new(format!("{:.1}%", rate * 100.0)),
        ]);

        // Display growth assumptions
        let growth_years: Vec<String> = growth_assumptions
            .iter()
            .enumerate()
            .map(|(i, &g)| format!("Year {}: {:.1}%", i + 1, g * 100.0))
            .collect();
        assumptions_table.add_row(vec![
            Cell::new("Growth Assumptions"),
            Cell::new(growth_years.join("\n")),
        ]);

        if let Some(cash_amt) = self.cash {
            assumptions_table.add_row(vec![
                Cell::new("Cash"),
                Cell::new(format!("${:.2}", cash_amt)),
            ]);
        }

        if let Some(debt_amt) = self.debt {
            assumptions_table.add_row(vec![
                Cell::new("Debt"),
                Cell::new(format!("${:.2}", debt_amt)),
            ]);
        }

        if let Some(prob) = self.probability_of_failure {
            assumptions_table.add_row(vec![
                Cell::new("Prob. of Failure"),
                Cell::new(format!("{:.1}%", prob * 100.0)),
            ]);
        }

        if let Some(shares) = self.shares_outstanding {
            assumptions_table.add_row(vec![
                Cell::new("Shares Outstanding"),
                Cell::new(format!("{}", shares)),
            ]);
        }

        // Table 2: CASH FLOW PROJECTIONS
        let mut cashflow_table = Table::new();
        cashflow_table.load_preset(presets::UTF8_BORDERS_ONLY);
        cashflow_table.set_header(vec!["Year", "FV", "PV"]);
        cashflow_table.set_constraints(vec![
            ColumnConstraint::Absolute(Width::Fixed(10)),
            ColumnConstraint::Absolute(Width::Fixed(10)),
            ColumnConstraint::Absolute(Width::Fixed(10)),
        ]);

        let mut year = 0;
        cashflow_table.add_row(vec![
            Cell::new(year),
            Cell::new(format!("{:.2}", 0.0)),
            Cell::new(format!("{:.2}", current_value)),
        ]);

        for assumption_rate in growth_assumptions.iter() {
            year += 1;
            current_value = current_value * (1. + assumption_rate);
            let pv_value = pv(rate, year, current_value);

            cashflow_table.add_row(vec![
                Cell::new(year),
                Cell::new(format!("{:.2}", current_value)),
                Cell::new(format!("{:.2}", pv_value)),
            ]);
            result += pv_value;
        }

        let last_growth = self.growth_assumptions.assumptions.last().unwrap();
        let terminal_value = current_value / (self.rate.unwrap() - last_growth) as f32;
        let sale_price = pv(rate, year, terminal_value);

        cashflow_table.add_row(vec![
            Cell::new("TV"),
            Cell::new(format!("{:.2}", terminal_value)),
            Cell::new(format!("{:.2}", sale_price)),
        ]);

        result += sale_price;

        cashflow_table.add_row(vec![
            Cell::new("NPV"),
            Cell::new(""),
            Cell::new(format!("${:.2}", result)),
        ]);

        // Table 3: ADJUSTMENTS
        let mut adjustments_table = Table::new();
        adjustments_table.load_preset(presets::UTF8_BORDERS_ONLY);
        adjustments_table.set_header(vec!["Adjustments", "Value"]);
        adjustments_table.set_constraints(vec![
            ColumnConstraint::Absolute(Width::Fixed(16)),
            ColumnConstraint::Absolute(Width::Fixed(16)),
        ]);

        adjustments_table.add_row(vec![
            Cell::new("NPV"),
            Cell::new(format!("${:.2}", result)),
        ]);

        if let Some(debt_amt) = self.debt {
            adjustments_table.add_row(vec![
                Cell::new("- Debt"),
                Cell::new(format!("${:.2}", debt_amt)),
            ]);
            result -= debt_amt;
        }

        if let Some(cash_amt) = self.cash {
            adjustments_table.add_row(vec![
                Cell::new("+ Cash"),
                Cell::new(format!("${:.2}", cash_amt)),
            ]);
            result += cash_amt;
        }

        if let Some(prob_failure) = self.probability_of_failure {
            let before_prob = result;
            adjustments_table.add_row(vec![
                Cell::new("× Prob. failure"),
                Cell::new(format!("{:.1}%", prob_failure * 100.0)),
            ]);
            result = before_prob * (1.0 - prob_failure);
        }

        adjustments_table.add_row(vec![
            Cell::new("= Intrinsic"),
            Cell::new(format!("${:.2}", result)),
        ]);

        if let Some(shares) = self.shares_outstanding {
            let value_per_share = result / shares as f32;
            adjustments_table.add_row(vec![Cell::new("")]);
            adjustments_table.add_row(vec![
                Cell::new("÷ Shares"),
                Cell::new(format!("{}", shares)),
            ]);
            adjustments_table.add_row(vec![
                Cell::new("= per Share"),
                Cell::new(format!("${:.2}", value_per_share)),
            ]);
        }

        // Print the three tables side by side
        let assumptions_str = assumptions_table.to_string();
        let cashflow_str = cashflow_table.to_string();
        let adjustments_str = adjustments_table.to_string();

        let assumptions_lines: Vec<&str> = assumptions_str.lines().collect();
        let cashflow_lines: Vec<&str> = cashflow_str.lines().collect();
        let adjustments_lines: Vec<&str> = adjustments_str.lines().collect();

        let max_lines = assumptions_lines
            .len()
            .max(cashflow_lines.len())
            .max(adjustments_lines.len());

        println!();
        for i in 0..max_lines {
            let assumptions_line = assumptions_lines.get(i).unwrap_or(&"");
            let cashflow_line = cashflow_lines.get(i).unwrap_or(&"");
            let adjustments_line = adjustments_lines.get(i).unwrap_or(&"");

            // Each table is ~36 chars wide, pad for alignment
            let padded_assumptions = format!("{:36}", assumptions_line);
            let padded_cashflow = format!("{:36}", cashflow_line);

            println!(
                "{}  {}  {}",
                padded_assumptions, padded_cashflow, adjustments_line
            );
        }
        println!();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intrisic_builder() {
        let expected = IntrinsicBuilder {
            current_value: Some(15.0),
            rate: Some(0.15),
            growth_assumptions: GrowthAssumptionBuilder::new()
                .add(GrowthAssumption(10, 0.05, None)),
            cash: None,
            debt: None,
            probability_of_failure: None,
            shares_outstanding: None,
        };

        let builded = IntrinsicBuilder::new()
            .add_current_value(15.0)
            .add_rate(0.15)
            .add_growth_assumptions(
                GrowthAssumptionBuilder::new().add(GrowthAssumption(10, 0.05, None)),
            );

        assert_eq!(expected, builded);
    }

    #[test]
    fn test_compute_on_simple_example() {
        let intrisic = IntrinsicBuilder::new()
            .add_current_value(15.0)
            .add_rate(0.15)
            .add_growth_assumptions(
                GrowthAssumptionBuilder::new().add(GrowthAssumption(10, 0.05, None)),
            );

        assert_eq!(intrisic.execute(), 184.67798);
    }
}
