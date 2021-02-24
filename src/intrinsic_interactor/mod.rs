use crate::{
    money::Money,
    stats_provider::provider::{KeyStats, Provider},
};
use growth_assumption::{GrowthAssumption, GrowthAssumptionBuilder};
use serde::Serialize;
use tokio::join;

pub mod growth_assumption;
pub mod intrinsic_builder;

#[derive(Debug, Serialize)]
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
}

impl IntrinsicInteractor {
    pub fn new(provider: Box<dyn Provider>) -> Self {
        Self { provider }
    }

    pub async fn execute(&self, ticker: &str) -> Result<Stats, Box<dyn std::error::Error>> {
        let (cash_flow, key_stats, analysis) = join!(
            self.provider.get_cash_flow(ticker),
            self.provider.get_key_stats(ticker),
            self.provider.get_analysis(ticker)
        );

        let key_stats_cp = key_stats.unwrap_or(KeyStats {
            total_cash: Money(0),
            market_cap: Money(0),
        });
        let cash_flow_cp = cash_flow.unwrap_or(Money(0));
        let analysis_cp = analysis.unwrap_or(0.0);

        let intrinsic = intrinsic_builder::IntrinsicBuilder::new()
            .add_cash(key_stats_cp.total_cash.clone())
            .add_fcf(cash_flow_cp.clone())
            .add_growth_assumptions(
                growth_assumption::GrowthAssumptionBuilder::new()
                    .add(GrowthAssumption(5, analysis_cp, None))
                    .add(GrowthAssumption(5, analysis_cp, Some(-0.01))),
            )
            .execute();

        Ok(Stats {
            ticker: ticker.clone().to_string(),
            analysis: analysis_cp,
            market_cap: key_stats_cp.market_cap,
            cash: key_stats_cp.total_cash,
            free_cash_flow: cash_flow_cp,
            intrinsic,
        })
    }
}
