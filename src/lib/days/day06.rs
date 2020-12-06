use std::collections::HashSet;

use crate::lib::advent::Result;
use crate::Day;

struct Group {
    people: Vec<HashSet<char>>,
}

impl Group {
    fn new(s: &str) -> Self {
        Self {
            people: s.lines().map(|c| c.chars().collect()).collect(),
        }
    }

    fn questions_answered(&self) -> usize {
        self.people
            .iter()
            .fold(HashSet::new(), |acc, q| acc.union(q).cloned().collect())
            .len()
    }

    fn questions_all_answered(&self) -> usize {
        let (p, start) = self.people.split_first().unwrap();
        let p = p.clone();
        start
            .iter()
            .fold(p, |acc, q| acc.intersection(q).cloned().collect())
            .len()
    }
}

#[derive(Default)]
pub struct Day06 {
    groups: Vec<Group>,
}

impl<'a> Day<'a> for Day06 {
    fn setup(&mut self, input: &'a str) -> Result<()> {
        self.groups = input.split("\n\n").map(Group::new).collect();
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let count: usize = self.groups.iter().map(|g| g.questions_answered()).sum();
        Ok(count.to_string())
    }

    fn part2(&self) -> Result<String> {
        let count: usize = self.groups.iter().map(|g| g.questions_all_answered()).sum();
        Ok(count.to_string())
    }
}
