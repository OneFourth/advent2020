use crate::lib::advent::Result;
use crate::Day;

#[derive(Default, Clone, Eq, PartialEq)]
struct ChairSim {
    chairs: Vec<Vec<u8>>,
}

impl ChairSim {
    fn new(s: &str) -> Self {
        Self {
            chairs: s.lines().map(Into::into).collect(),
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<&u8> {
        if x < 0 || y < 0 {
            None
        } else {
            self.chairs.get(y as usize).and_then(|s| s.get(x as usize))
        }
    }

    fn is_occupied_distance(&self, (x, y): (usize, usize), (dx, dy): (isize, isize)) -> bool {
        (1..)
            .map(|i| {
                let nx = x as isize + i * dx;
                let ny = y as isize + i * dy;

                self.get(nx, ny)
            })
            .take_while(|&c| c.is_some())
            .find(|c| *c != Some(&b'.'))
            .map(|c| c == Some(&b'#'))
            .unwrap_or(false)
    }

    fn apply_part1(&mut self) -> bool {
        let mut new_chairs = self.chairs.clone();

        for (y, s) in self.chairs.iter().enumerate() {
            for (x, c) in s.iter().enumerate() {
                let total = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ]
                .iter()
                .filter(|(dx, dy)| self.get(x as isize + dx, y as isize + dy) == Some(&b'#'))
                .count();

                let new_c = new_chairs.get_mut(y).and_then(|s| s.get_mut(x)).unwrap();

                match (c, total) {
                    (b'L', 0) => *new_c = b'#',
                    (b'#', 5..=8) => *new_c = b'L',
                    _ => {}
                }
            }
        }

        let result = self.chairs != new_chairs;
        self.chairs = new_chairs;
        result
    }

    fn apply_part2(&mut self) -> bool {
        let mut new_chairs = self.chairs.clone();

        for (y, s) in self.chairs.iter().enumerate() {
            for (x, c) in s.iter().enumerate() {
                let total = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ]
                .iter()
                .filter(|&&d| self.is_occupied_distance((x, y), d))
                .count();

                let new_c = new_chairs.get_mut(y).and_then(|s| s.get_mut(x)).unwrap();

                match (c, total) {
                    (b'L', 0) => *new_c = b'#',
                    (b'#', 5..=8) => *new_c = b'L',
                    _ => {}
                }
            }
        }

        let result = self.chairs != new_chairs;
        self.chairs = new_chairs;
        result
    }

    fn count_occupied(&self) -> usize {
        self.chairs
            .iter()
            .map(|s| s.iter().filter(|&&c| c == b'#').count())
            .sum()
    }
}

#[derive(Default)]
pub struct Day11 {
    sim: ChairSim,
}

impl<'a> Day<'a> for Day11 {
    fn setup(&mut self, input: &'a str) -> Result<()> {
        self.sim = ChairSim::new(input);
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let mut sim = self.sim.clone();
        while sim.apply_part1() {}
        Ok(sim.count_occupied().to_string())
    }

    fn part2(&self) -> Result<String> {
        let mut sim = self.sim.clone();
        while sim.apply_part2() {}
        Ok(sim.count_occupied().to_string())
    }
}
