use crate::lib::advent::Result;
use crate::lib::utils::parse_lines;
use crate::Day;

#[derive(Default)]
pub struct Day01 {
    nums: Vec<usize>,
}

impl<'a> Day<'a> for Day01 {
    fn setup(&mut self, input: &str) -> Result<()> {
        self.nums = parse_lines(&input)?;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        for (n, x) in self.nums.iter().enumerate() {
            for y in self.nums.iter().skip(n) {
                if x + y == 2020 {
                    let result = x * y;
                    return Ok(result.to_string());
                }
            }
        }
        Err("No numbers found".into())
    }

    fn part2(&self) -> Result<String> {
        for (n, x) in self.nums.iter().enumerate() {
            for (m, y) in self.nums.iter().skip(n).enumerate() {
                for z in self.nums.iter().skip(m) {
                    if x + y + z == 2020 {
                        let result = x * y * z;
                        return Ok(result.to_string());
                    }
                }
            }
        }
        Err("No numbers found".into())
    }
}

#[cfg(test)]
mod tests {
    use super::Day01;
    use crate::lib::advent::Day;

    #[test]
    fn example() {
        let s = "\
1721
979
366
299
675
1456";
        let mut d = Day01::default();
        d.setup(s).unwrap();
        assert_eq!("514579", d.part1().unwrap());
        assert_eq!("241861950", d.part2().unwrap());
    }
}
