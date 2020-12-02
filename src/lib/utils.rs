use std::str::FromStr;

pub fn parse_lines<T>(input: &str) -> Vec<T>
where
    T: FromStr,
{
    input.lines().map(|s| s.parse().ok().unwrap()).collect()
}
