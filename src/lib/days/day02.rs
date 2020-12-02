use crate::lib::advent::Result;
use crate::Day;
use regex::Regex;

#[derive(Default)]
pub struct Day02 {
    rules: Vec<(usize, usize, char, String)>,
}

impl Day for Day02 {
    fn number(&self) -> u8 {
        2
    }

    fn setup(&mut self, input: String) -> Result {
        let r = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
        for line in input.lines() {
            for cap in r.captures_iter(&line) {
                self.rules.push((
                    cap[1].parse().unwrap(),
                    cap[2].parse().unwrap(),
                    cap[3].parse().unwrap(),
                    cap[4].parse().unwrap(),
                ));
            }
        }
        Ok(())
    }

    fn part1(&self) -> Result {
        let mut valid = 0;
        for (min, max, letter, password) in &self.rules {
            let count = password.matches(|c| c == *letter).count();
            if count >= *min && count <= *max {
                valid += 1;
            }
        }
        dbg!(valid);
        Ok(())
    }

    fn part2(&self) -> Result {
        let mut valid = 0;
        for (lower, upper, letter, password) in &self.rules {
            if (password.chars().nth(*lower - 1).unwrap() == *letter)
                ^ (password.chars().nth(*upper - 1).unwrap() == *letter)
            {
                valid += 1;
            }
        }
        dbg!(valid);
        Ok(())
    }
}
