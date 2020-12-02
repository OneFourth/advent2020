use crate::lib::advent::Result;
use crate::Day;
use regex::Regex;

struct Rule {
    left: usize,
    right: usize,
    letter: char,
    password: String,
}

impl Rule {
    fn new(left: usize, right: usize, letter: char, password: String) -> Result<Self> {
        if password.len() >= left && password.len() >= right && left < right {
            Ok(Rule {
                left,
                right,
                letter,
                password,
            })
        } else {
            Err(format!("Invalid range {}-{} for password {}", left, right, password).into())
        }
    }

    fn valid_part1(&self) -> bool {
        let count = self.password.matches(|c| c == self.letter).count();

        self.left <= count && count <= self.right
    }

    fn get_char(&self, i: usize) -> Result<char> {
        self.password
            .chars()
            .nth(i)
            .ok_or_else(|| "Invalid index".into())
    }

    fn valid_part2(&self) -> bool {
        let lchar = self.get_char(self.left - 1).unwrap();
        let rchar = self.get_char(self.right - 1).unwrap();

        (lchar == self.letter) ^ (rchar == self.letter)
    }
}

#[derive(Default)]
pub struct Day02 {
    rules: Vec<Rule>,
}

impl Day for Day02 {
    fn number(&self) -> u8 {
        2
    }

    fn setup(&mut self, input: &str) -> Result<()> {
        let r = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$")?;
        for line in input.lines() {
            for cap in r.captures_iter(&line) {
                let r = Rule::new(
                    cap[1].parse()?,
                    cap[2].parse()?,
                    cap[3].parse()?,
                    cap[4].parse()?,
                )?;

                self.rules.push(r);
            }
        }
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let valid = self.rules.iter().filter(|r| r.valid_part1()).count();
        Ok(valid.to_string())
    }

    fn part2(&self) -> Result<String> {
        let valid = self.rules.iter().filter(|r| r.valid_part2()).count();
        Ok(valid.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Day02;
    use crate::lib::advent::Day;

    #[test]
    fn example() {
        let s = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";
        let mut d = Day02::default();
        d.setup(s).unwrap();
        assert_eq!("2", d.part1().unwrap());
        assert_eq!("1", d.part2().unwrap());
    }
}
