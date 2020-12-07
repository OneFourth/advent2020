use std::collections::HashMap;

use regex::Regex;

use crate::lib::advent::Result;
use crate::Day;

#[derive(Debug, Default)]
struct BagRules<'a> {
    rules: HashMap<&'a str, Vec<(usize, &'a str)>>,
}

impl<'a> BagRules<'a> {
    fn new(input: &'a str) -> Self {
        let r = Regex::new(r"(?:(\d+) )?(\w+ \w+) bags?").unwrap();
        let rules = input
            .lines()
            .map(|l| {
                let mut key = "";
                let mut values = Vec::new();
                for caps in r.captures_iter(l) {
                    if let Some(d) = caps.get(1) {
                        values.push((d.as_str().parse().unwrap(), caps.get(2).unwrap().as_str()));
                    } else {
                        let name = caps.get(2).unwrap().as_str();
                        match name {
                            "no other" => {}
                            _ => key = name,
                        }
                    }
                }
                (key, values)
            })
            .collect();

        Self { rules }
    }

    fn can_hold(&self, bag: &str, target: &str) -> bool {
        if let Some(v) = self.rules.get(bag) {
            v.iter()
                .any(|(_, b)| *b == target || self.can_hold(b, target))
        } else {
            false
        }
    }

    fn count_rules(&self, held: &str) -> usize {
        self.rules
            .iter()
            .filter(|(b, _)| self.can_hold(b, held))
            .count()
    }

    fn count_bags(&self, bag: &str) -> usize {
        if let Some(v) = self.rules.get(bag) {
            v.iter()
                .map(|(count, other)| count + (count * self.count_bags(other)))
                .sum()
        } else {
            0
        }
    }
}

#[derive(Default)]
pub struct Day07<'a> {
    bag_rules: BagRules<'a>,
}

impl<'a> Day<'a> for Day07<'a> {
    fn setup(&mut self, input: &'a str) -> Result<()> {
        self.bag_rules = BagRules::new(input);

        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(self.bag_rules.count_rules("shiny gold").to_string())
    }

    fn part2(&self) -> Result<String> {
        Ok(self.bag_rules.count_bags("shiny gold").to_string())
    }
}
