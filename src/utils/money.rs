use core::{f32, fmt};
use serde::Serialize;
use std::ops::{Add, AddAssign, Div, Mul};
use std::{convert::TryFrom, fmt::Display};

const THOUSAND: i64 = 1000;
const MILION: i64 = 1000000;
const BILION: i64 = 1000000000;

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Money(pub i64);

impl Div<f32> for Money {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Money((self.0 as f32 / rhs).round() as i64)
    }
}

impl Div<Money> for Money {
    type Output = f32;

    fn div(self, rhs: Money) -> Self::Output {
        self.0 as f32 / rhs.0 as f32
    }
}

impl Mul<f32> for Money {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Money((self.0 as f32 * rhs).round() as i64)
    }
}

impl Add<Self> for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Money(self.0 + rhs.0)
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0 + other.0;
    }
}

impl TryFrom<f32> for Money {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Money(value as i64))
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str = self.0.to_string();
        let vec_chars = as_str.chars().rev().collect::<Vec<char>>();
        let res = vec_chars
            .chunks(3)
            .map(|chs| chs.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(",")
            .chars()
            .rev()
            .collect::<String>();

        write!(f, "{}", res)
    }
}

impl TryFrom<String> for Money {
    type Error = &'static str;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        let multiplier = get_multiplier(&value);
        cleanup_letters(&mut value);

        if value.contains(".") {
            Ok(handle_int_value(value, multiplier).unwrap())
        } else {
            Ok(handle_float_value(value, multiplier).unwrap())
        }
    }
}

fn handle_float_value(value: String, multiplier: i64) -> Result<Money, Box<dyn std::error::Error>> {
    let maybe_value = value.parse::<i64>();
    if maybe_value.is_ok() {
        Ok(Money(maybe_value.unwrap() * multiplier))
    } else {
        Ok(Money(0))
    }
}

fn handle_int_value(value: String, multiplier: i64) -> Result<Money, Box<dyn std::error::Error>> {
    let maybe_value = value.parse::<f32>();
    if maybe_value.is_ok() {
        let value = maybe_value.unwrap();
        let result = clean_up_after_zero((value * multiplier as f32) as i64);
        Ok(Money(result))
    } else {
        Ok(Money(0))
    }
}

fn get_multiplier(value: &str) -> i64 {
    if value.contains("K") {
        THOUSAND
    } else if value.contains("M") {
        MILION
    } else if value.contains("B") {
        BILION
    } else {
        1
    }
}

fn cleanup_letters(value: &mut String) {
    *value = value
        .replace(",", "")
        .replace("K", "")
        .replace("M", "")
        .replace("B", "");
}

fn clean_up_after_zero(val: i64) -> i64 {
    let as_str = val.to_string();
    let length = as_str.len();
    let first_zero = val.to_string().find("0").unwrap_or(3);

    let substr = &as_str[0..first_zero];

    format!("{}{}", substr, "0".repeat(length - substr.len()))
        .parse::<i64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_creation_from_f32() {
        assert_eq!(Money::try_from(100.0).unwrap_or(Money(-1)), Money(100));
    }

    #[test]
    fn test_money_creation_from_string() {
        assert_eq!(
            Money::try_from("100".to_string()).unwrap_or(Money(-1)),
            Money(100)
        )
    }

    #[test]
    fn test_money_creation_from_smart_strings() {
        let tests = vec![
            ("1B".to_string(), 1000000000),
            ("1M".to_string(), 1000000),
            ("1K".to_string(), 1000),
            ("1.56K".to_string(), 1560),
            ("1.4M".to_string(), 1400000),
            ("232.46B".to_string(), 232460000000),
        ];

        for test in tests.iter() {
            let (input, expect) = test;
            assert_eq!(
                Money::try_from(input.clone()).unwrap_or(Money(-1)),
                Money(*expect)
            );
        }
    }

    #[test]
    fn test_clean_up_every_number_after_zero() {
        let tests = vec![(1325000, 1325000), (1325053, 1325000)];

        for test in tests.iter() {
            let (input, expect) = test;
            assert_eq!(clean_up_after_zero(*input), *expect);
        }
    }

    #[test]
    fn test_money_mul_floats() {
        let money = Money(100);

        assert_eq!(money * 10.65, Money(1065))
    }

    #[test]
    fn test_from_broken_string_should_return_0() {
        let money = Money::try_from("N/A".to_string()).unwrap();
        assert_eq!(money, Money(0));
    }

    #[test]
    fn test_display_money_format() {
        let money = Money(1000);
        assert_eq!(format!("{}", money), "1,000".to_string());
    }
}