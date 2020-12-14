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

pub fn between(input: &str, lhs: char, rhs: char) -> Result<&str> {
    let l = input.find(lhs).ok_or("Could not find [")? + 1;
    let r = input.find(rhs).ok_or("Could not find ]")?;
    Ok(&input[l..r])
}
