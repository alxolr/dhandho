use crate::{money::Money, stats_provider::provider::Provider};
use growth_assumption::{GrowthAssumption, GrowthAssumptionBuilder};
use tokio::join;

pub mod growth_assumption;
pub mod intrinsic_builder;

#[derive(Debug)]
pub struct Stats {
    ticker: String,
    cash: Money,
    free_cash_flow: Money,
    analysis: f32,
    market_cap: Money,
    intrinsic: Money,
}

pub struct IntrinsicInteractor {
    provider: Box<dyn Provider>,
    ticker: String,
}

impl IntrinsicInteractor {
    pub fn new(ticker: String, provider: Box<dyn Provider>) -> Self {
        Self { ticker, provider }
    }

    pub async fn execute(&self) -> Result<Stats, Box<dyn std::error::Error>> {
        let (cash_flow, key_stats, analysis) = join!(
            self.provider.get_cash_flow(),
            self.provider.get_key_stats(),
            self.provider.get_analysis()
        );

        let key_stats_cp = key_stats?;
        let cash_flow_cp = cash_flow?;
        let analysis_cp = analysis?;

        let intrinsic = intrinsic_builder::IntrinsicBuilder::new()
            .add_cash(key_stats_cp.total_cash.clone())
            .add_fcf(cash_flow_cp.clone())
            .add_rate(0.15)
            .add_growth_assumptions(
                growth_assumption::GrowthAssumptionBuilder::new()
                    .add(GrowthAssumption(5, analysis_cp, None))
                    .add(GrowthAssumption(5, analysis_cp, Some(-0.01))),
            )
            .compute();

        Ok(Stats {
            ticker: self.ticker.clone(),
            analysis: analysis_cp,
            market_cap: key_stats_cp.market_cap,
            cash: key_stats_cp.total_cash,
            free_cash_flow: cash_flow_cp,
            intrinsic
        })
    }
}
