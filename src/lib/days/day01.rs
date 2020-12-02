use crate::lib::advent::Result;
use crate::lib::utils::parse_lines;
use crate::Day;

#[derive(Default)]
pub struct Day01 {
    nums: Vec<usize>,
}

impl Day for Day01 {
    fn number(&self) -> u8 {
        1
    }

    fn setup(&mut self, input: String) -> Result {
        self.nums = parse_lines(&input);
        Ok(())
    }

    fn part1(&self) -> Result {
        for (n, x) in self.nums.iter().enumerate() {
            for y in self.nums.iter().skip(n) {
                if x + y == 2020 {
                    dbg!(x * y);
                    return Ok(());
                }
            }
        }
        Err("No numbers found".into())
    }

    fn part2(&self) -> Result {
        for (n, x) in self.nums.iter().enumerate() {
            for (m, y) in self.nums.iter().skip(n).enumerate() {
                for z in self.nums.iter().skip(m) {
                    if x + y + z == 2020 {
                        dbg!(x * y * z);
                        return Ok(());
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
        let s = "1721
979
366
299
675
1456"
            .to_string();
        let mut d = Day01::default();
        d.setup(s).unwrap();
        d.part1().unwrap();
        d.part2().unwrap();
    }
}
