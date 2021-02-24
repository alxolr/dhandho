use std::convert::TryFrom;

use super::provider::*;
use crate::money::Money;
use async_trait::async_trait;
use reqwest::get;
use select::{document::Document, node::Node, predicate};
use tendril::StrTendril;

pub struct Yahoo {
    url: String,
}

impl Yahoo {
    pub fn new() -> Self {
        Yahoo {
            url: "https://finance.yahoo.com/quote".to_string(),
        }
    }
}

#[async_trait]
impl Provider for Yahoo {
    async fn get_analysis(&self, ticker: &str) -> Result<f32, Box<dyn std::error::Error>> {
        let url = format!("{}/{}/analysis?p={}", self.url, ticker, ticker);
        let body = get(&url).await?.text().await?;
        let document = Document::from(StrTendril::from(body));
        let values = document
            .find(predicate::Name("span"))
            .filter(|i| i.text().contains("Next 5 Years"))
            .next()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .descendants()
            .filter(|i| i.text().contains("%"))
            .collect::<Vec<Node>>()
            .first()
            .unwrap()
            .text();

        let value = values.replace("%", "").parse::<f32>().unwrap() / 100.0;

        Ok(value)
    }

    async fn get_cash_flow(&self, ticker: &str) -> Result<Money, Box<dyn std::error::Error>> {
        let url = format!("{}/{}/cash-flow?p={}", self.url, ticker, ticker);
        let body = get(&url).await?.text().await?;
        let value = Document::from(StrTendril::from(body))
            .find(predicate::Name("span"))
            .filter(|i| i.text().contains("Free"))
            .next()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .descendants()
            .filter(|n| n.attr("data-test").unwrap_or("false") == "fin-col")
            .map(|n| n.text())
            .filter(|n| n.len() > 1)
            .map(|n| Money::try_from(n.clone()).unwrap() * 1000.0) // all stats are in thousands
            .collect::<Vec<Money>>()
            .first()
            .unwrap()
            .clone();

        Ok(value)
    }

    async fn get_key_stats(&self, ticker: &str) -> Result<KeyStats, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/{}/key-statistics?p={}",
            self.url, ticker, ticker
        );
        let body = get(&url).await?.text().await?;
        let document = Document::from(StrTendril::from(body));

        let total_cash = extract_field(&document, "Total Cash");
        let market_cap = extract_field(&document, "Market Cap");

        Ok(KeyStats {
            total_cash: Money::try_from(total_cash)?,
            market_cap: Money::try_from(market_cap)?,
        })
    }
}

fn extract_field(document: &Document, field: &str) -> String {
    let result = document
        .find(predicate::Name("span"))
        .filter(|i| i.text().contains(field))
        .next()
        .unwrap()
        .parent() // td
        .unwrap()
        .parent()
        .unwrap()
        .descendants()
        .last()
        .unwrap()
        .text();

    result
}
