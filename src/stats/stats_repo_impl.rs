use super::repo::*;
use serde::Deserialize;
use serde::*;
use std::{convert::TryInto, env};

use crate::utils::money::Money;
use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
pub struct StatsRepoImpl {}

impl StatsRepoImpl {
    pub fn new() -> Self {
        StatsRepoImpl {}
    }
}

#[async_trait]
impl StatsRepo for StatsRepoImpl {
    async fn get_stats(&self, ticker: &str) -> Result<Stats, Box<dyn std::error::Error>> {
        let api_host_host = env::var("X_RAPIDAPI_HOST")?;
        let api_key = env::var("X_RAPIDAPI_KEY")?;
        let mut headers = HeaderMap::new();

        headers.append("x-rapidapi-key", HeaderValue::from_str(&api_key)?);
        headers.append("x-rapidapi-host", HeaderValue::from_str(&api_host_host)?);

        let client = Client::new();

        let response = client
            .get(&format!(
                "https://{}/stock/v2/get-analysis?symbol={}&regions=US",
                api_host_host, ticker
            ))
            .headers(headers)
            .send()
            .await?
            .json::<Analysis>()
            .await?;

        println!("Analysis {:?}", response);

        Ok(Stats {
            free_cash_flow: Money(100),
            growth_analysis: 0.15,
            total_cash: Money(100),
            market_cap: Money(100),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Analysis {
    symbol: String,
    financial_data: FinancialData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FinancialData {
    free_cashflow: FreeCashflow,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FreeCashflow {
    raw: i64,
}
