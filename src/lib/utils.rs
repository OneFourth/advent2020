use crate::lib::advent::Result;
use std::str::FromStr;

pub fn parse_lines<T>(input: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    input
        .lines()
        .map(|s| s.parse().map_err(|_| "Failed to parse line".into()))
        .collect()
}
