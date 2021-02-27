use async_trait::async_trait;

use crate::utils::money::Money;

#[derive(Debug)]
pub struct Stats {
    pub growth_analysis: f32,
    pub free_cash_flow: Money,
    pub total_cash: Money,
    pub market_cap: Money,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait StatsRepo {
    async fn get_stats(&self, ticker: &str) -> Result<Stats, Box<dyn std::error::Error>>;
}
