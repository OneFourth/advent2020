use crate::lib::advent::Result;
use crate::lib::utils::parse_lines;
use crate::Day;

#[derive(Default)]
pub struct Day09 {
    numbers: Vec<isize>,
}

impl<'a> Day<'a> for Day09 {
    fn setup(&mut self, input: &'a str) -> Result<()> {
        self.numbers = parse_lines(input)?;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let v = self
            .numbers
            .windows(25)
            .zip(self.numbers.iter().skip(25))
            .find(|(w, n)| !w.iter().any(|a| w.contains(&(*n - *a))))
            .ok_or("Could not find number")?;
        Ok(v.1.to_string())
    }

    fn part2(&self) -> Result<String> {
        let target: isize = self.part1()?.parse()?;
        let mut n = 2;
        let mut result = 0;
        while n < self.numbers.len() {
            if let Some(r) = self.numbers.windows(n).find(|ns| ns.iter().sum::<isize>() == target) {
                result = r.iter().min().unwrap() + r.iter().max().unwrap();
            }

            n += 1;
        }
        Ok(result.to_string())
    }
}
