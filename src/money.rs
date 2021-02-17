use core::f32;
use std::convert::TryFrom;

const THOUSAND: i64 = 1000;
const MILION: i64 = 1000000;
const BILION: i64 = 1000000000;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Money(i64);

impl TryFrom<f32> for Money {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Money(value as i64))
    }
}

impl TryFrom<String> for Money {
    type Error = &'static str;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        let multiplier = get_multiplier(&value);
        cleanup_letters(&mut value);

        if value.contains(".") {
            let maybe_value = value.parse::<f32>();
            if maybe_value.is_ok() {
                let value = maybe_value.unwrap();
                let result = clean_up_after_zero((value * multiplier as f32) as i64);
                return Ok(Money(result));
            } else {
                Err("Couldn't parse the provided string")
            }
        } else {
            let maybe_value = value.parse::<i64>();
            if maybe_value.is_err() {
                Err("Couldn't parse the provided string")
            } else {
                Ok(Money(maybe_value.unwrap() * multiplier))
            }
        }
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
    let first_zero = val.to_string().find("0").unwrap();
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
}
