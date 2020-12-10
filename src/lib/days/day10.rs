use crate::lib::{advent::Result, utils::parse_lines};
use crate::Day;

#[derive(Default)]
pub struct Day10 {
    numbers: Vec<usize>,
}

fn length_to_count(len: usize) -> usize {
    (0..(2u32.pow(len as u32)))
        .map(|v| format!("{:01$b}", v, len))
        .filter(|s| !s.contains("000"))
        .count()
}

impl<'a> Day<'a> for Day10 {
    fn setup(&mut self, input: &'a str) -> Result<()> {
        self.numbers = parse_lines(input)?;
        self.numbers.push(0);
        self.numbers.sort();
        self.numbers.push(self.numbers.last().unwrap() + 3);
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let mut differences = [0, 0, 0];
        for v in self.numbers.windows(2) {
            if let [a, b] = v {
                *differences.get_mut(b - a - 1).ok_or("invalid difference")? += 1;
            }
        }
        Ok((differences[0] * differences[2]).to_string())
    }

    fn part2(&self) -> Result<String> {
        let diffs: Vec<_> = self
            .numbers
            .windows(2)
            .filter_map(|v| if let [a, b] = v { Some(b - a) } else { None })
            .collect();

        let result: usize = diffs
            .split(|&v| v != 1)
            .filter_map(|arr| match arr.len() {
                0 | 1 => None,
                l => Some(length_to_count(l - 1)),
            })
            .product();

        Ok(result.to_string())
    }
}
