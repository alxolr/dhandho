use crate::money::Money;
use async_trait::async_trait;

#[derive(Debug)]
pub struct KeyStats {
    pub total_cash: Money,
    pub market_cap: Money,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Provider {
    /*Get the Free Cash Flow*/
    async fn get_cash_flow(&self, ticker: &str) -> Result<Money, Box<dyn std::error::Error>>;

    /* Get the market capitalisation, company price */
    async fn get_key_stats(&self, ticker: &str) -> Result<KeyStats, Box<dyn std::error::Error>>;

    /* Get company analysis of company growth*/
    async fn get_analysis(&self, ticker: &str) -> Result<f32, Box<dyn std::error::Error>>;
}
