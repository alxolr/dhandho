use std::convert::TryFrom;

use super::provider::*;
use crate::money::Money;
use async_trait::async_trait;
use reqwest::get;
use select::{document::Document, node::Node, predicate};
use tendril::StrTendril;

pub struct Yahoo {
    url: String,
    ticker: String,
}

impl Yahoo {
    pub fn new(ticker: String) -> Self {
        Yahoo {
            url: "https://finance.yahoo.com/quote".to_string(),
            ticker,
        }
    }
}

#[async_trait]
impl Provider for Yahoo {
    async fn get_analysis(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let url = format!("{}/{}/analysis?p={}", self.url, self.ticker, self.ticker);
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

    async fn get_fcf(&self) -> Result<Vec<Money>, Box<dyn std::error::Error>> {
        let url = format!("{}/{}/cash-flow?p={}", self.url, self.ticker, self.ticker);
        let body = get(&url).await?.text().await?;
        let values = Document::from(StrTendril::from(body))
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
            .map(|n| Money::try_from(n.clone()).unwrap() * 1000.0)
            .collect::<Vec<Money>>();

        Ok(values)
    }

    async fn get_key_stats(&self) -> Result<KeyStats, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/{}/key-statistics?p={}",
            self.url, self.ticker, self.ticker
        );
        let body = get(&url).await?.text().await?;
        let document = Document::from(StrTendril::from(body));

        let cash = extract_field(&document, "Total Cash");
        let mkt_cap = extract_field(&document, "Market Cap");

        Ok(KeyStats::new(
            Money::try_from(cash)?,
            Money::try_from(mkt_cap)?,
        ))
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
