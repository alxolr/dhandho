use super::{growth_assumption_builder::{GrowthAssumption, GrowthAssumptionBuilder}, intrinsic_builder::IntrinsicBuilder};
use crate::{
    stats::repo::{Stats as ProviderStats, StatsRepo},
    utils::money::Money,
};
use serde::Serialize;
use tokio::join;

#[derive(Debug, Serialize)]
pub struct Intrinsic {
    ticker: String,
    total_cash: Money,
    free_cash_flow: Money,
    growth_analysis: f32,
    market_cap: Money,
    intrinsic: Money,
}

pub struct IntrinsicService {
    stats_repo: Box<dyn StatsRepo>,
}

impl IntrinsicService {
    pub fn new(stats_repo: Box<dyn StatsRepo>) -> Self {
        Self { stats_repo }
    }

    pub async fn execute(&self, ticker: &str) -> Result<Intrinsic, Box<dyn std::error::Error>> {
        let stats = self.stats_repo.get_stats(ticker).await?;

        let intrinsic = IntrinsicBuilder::new()
            .add_cash(stats.total_cash.clone())
            .add_fcf(stats.free_cash_flow.clone())
            .add_growth_assumptions(
                GrowthAssumptionBuilder::new()
                    .add(GrowthAssumption(5, stats.growth_analysis.clone(), None))
                    .add(GrowthAssumption(5, stats.growth_analysis.clone(), Some(-0.01))),
            )
            .execute();

        Ok(Intrinsic {
            ticker: ticker.clone().to_string(),
            growth_analysis: stats.growth_analysis,
            market_cap: stats.market_cap,
            total_cash: stats.total_cash,
            free_cash_flow: stats.free_cash_flow,
            intrinsic,
        })
    }
}
