use std::convert::TryFrom;
use std::error::Error;
use async_trait::async_trait;
use reqwest::blocking::get;
use select::{document, predicate};
use tendril::StrTendril;

use crate::money::Money;

const PROVIDER: &'static str = "https://finance.yahoo.com/quote";


#[async_trait]
trait Prospector {
    async fn get(&self) -> Vec<Money>;
}


#[derive(Debug, PartialEq)]
pub struct StatsProvider {
    ticker: String,
    pub free_cash_flows: Option<Vec<Money>>,
}

impl StatsProvider {
    pub fn new(ticker: String) -> Self {
        StatsProvider {
            ticker,
            free_cash_flows: None,
        }
    }

    pub fn gather(mut self) -> Result<Self, Box<dyn Error>> {
        self.free_cash_flows = Some(self.gather_free_cash_flows().unwrap());

        Ok(self)
    }

    fn gather_free_cash_flows(&self) -> Result<Vec<Money>, Box<dyn Error>> {
        let body = get(&format!(
            "{}/{}/cash-flow?p={}",
            PROVIDER, self.ticker, self.ticker
        ))?
        .text()?;

        let document = document::Document::from(StrTendril::from(body));
        let values = document
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
            .map(|n| Money::try_from(n.clone()).unwrap())
            .collect::<Vec<Money>>();

        Ok(values)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use mocktopus::mocking::*;

//     #[test]
//     fn test_gather_free_cash_flows() {
//         StatsProvider::gather_free_cash_flows
//             .mock_safe(|_| MockResult::Return(Ok(vec![Money(100)])));

//         assert_eq!(
//             StatsProvider::new("GPP".to_string()).gather().unwrap(),
//             StatsProvider {
//                 ticker: "GPP".to_string(),
//                 free_cash_flows: Some(vec![Money(100)])
//             }
//         )
//     }
// }
