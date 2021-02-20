use crate::money::Money;
use async_trait::async_trait;

#[derive(Debug)]
pub struct KeyStats {
    total_cash: Money,
    mkt_cap: Money,
}

impl KeyStats {
    pub fn new(total_cash: Money, mkt_cap: Money) -> Self {
        KeyStats { total_cash, mkt_cap }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Scraper {
    /*Get the Free Cash Flow*/
    async fn get_fcf(&self) -> Result<Vec<Money>, Box<dyn std::error::Error>>;

    /* Get the market capitalisation, company price */
    async fn get_key_stats(&self) -> Result<KeyStats, Box<dyn std::error::Error>>;
}
