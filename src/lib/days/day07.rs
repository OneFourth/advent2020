use std::collections::HashMap;

use regex::Regex;

use crate::lib::advent::Result;
use crate::Day;

#[derive(Debug, Default)]
struct BagRules<'a> {
    rules: HashMap<&'a str, Vec<(usize, &'a str)>>,
}

impl<'a> BagRules<'a> {
    fn new(input: &'a str) -> Result<Self> {
        let r = Regex::new(r"(?:(\d+) )?(\w+ \w+) bags?").unwrap();
        let mut rules = HashMap::new();

        for line in input.lines() {
            let mut key = "";
            let mut values = Vec::new();
            for caps in r.captures_iter(line) {
                if let Some(d) = caps.get(1) {
                    values.push((
                        d.as_str().parse()?,
                        caps.get(2).ok_or("Failed to parse")?.as_str(),
                    ));
                } else {
                    let name = caps.get(2).ok_or("Failed to parse")?.as_str();
                    match name {
                        "no other" => {}
                        _ => key = name,
                    }
                }
            }

            rules.insert(key, values);
        }

        Ok(Self { rules })
    }

    fn can_hold(&self, bag: &str, target: &str) -> bool {
        let rule = &self.rules[bag];

        rule.iter().any(|(_, b)| *b == target) || rule.iter().any(|(_, b)| self.can_hold(b, target))
    }

    fn count_rules(&self, held: &str) -> usize {
        self.rules
            .iter()
            .filter(|(b, _)| self.can_hold(b, held))
            .count()
    }

    fn count_bags(&self, bag: &str) -> usize {
        self.rules[bag]
            .iter()
            .map(|(count, other)| count + (count * self.count_bags(other)))
            .sum()
    }
}

#[derive(Default)]
pub struct Day07<'a> {
    bag_rules: BagRules<'a>,
}

impl<'a> Day<'a> for Day07<'a> {
    fn setup(&mut self, input: &'a str) -> Result<()> {
        self.bag_rules = BagRules::new(input)?;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(self.bag_rules.count_rules("shiny gold").to_string())
    }

    fn part2(&self) -> Result<String> {
        Ok(self.bag_rules.count_bags("shiny gold").to_string())
    }
}
