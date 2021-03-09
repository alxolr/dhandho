use super::repo::*;
use serde::Deserialize;
use std::env;

use crate::utils::{growth_rate, money::Money};
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

        let analysis = client
            .get(&format!(
                "https://{}/stock/v2/get-cash-flow?symbol={}&regions=US",
                api_host_host, ticker
            ))
            .headers(headers)
            .send()
            .await?
            .json::<Analysis>()
            .await?;

        let first_cashflow = analysis
            .cashflow_statement_history
            .cashflow_statements
            .first()
            .unwrap();

        let first_balance_sheet = analysis
            .balance_sheet_history
            .balance_sheet_statements
            .first()
            .unwrap();

        Ok(Stats {
            free_cashflow: Money(first_cashflow.free_cashflow()),
            growth_analysis: compute_growth_rate(analysis.cashflow_statement_history),
            total_cash: Money(first_balance_sheet.cash.raw),
            market_cap: Money(analysis.price.market_cap.raw),
        })
    }
}

fn compute_growth_rate(cashflow_statement_history: CashflowStatementHistory) -> f32 {
    let fcfs = cashflow_statement_history
        .cashflow_statements
        .iter()
        .map(|cashflow| cashflow.free_cashflow().clone())
        .map(|free_cashflow| Money(free_cashflow))
        .collect::<Vec<_>>();

        growth_rate::get_growth_rate(fcfs)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Analysis {
    symbol: String,
    cashflow_statement_history: CashflowStatementHistory,
    balance_sheet_history: BalanceSheetHistory,
    price: Price,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CashflowStatementHistory {
    cashflow_statements: Vec<CashflowStatement>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BalanceSheetHistory {
    balance_sheet_statements: Vec<BalanceSheetStatement>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CashflowStatement {
    net_income: NetIncome,
    depreciation: Depreciation,
    capital_expenditures: CapitalExpenditures,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BalanceSheetStatement {
    cash: Cash,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NetIncome {
    raw: i64,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Depreciation {
    raw: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CapitalExpenditures {
    raw: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Price {
    market_cap: MarketCap,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MarketCap {
    raw: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Cash {
    raw: i64,
}

impl CashflowStatement {
    fn free_cashflow(&self) -> i64 {
        self.net_income.raw + self.depreciation.raw + self.capital_expenditures.raw
    }
}
