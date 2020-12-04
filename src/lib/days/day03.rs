use crate::lib::advent::Result;
use crate::Day;

#[derive(Default)]
struct Map {
    data: Vec<String>,
    width: usize,
    height: usize,
}

#[derive(Default)]
pub struct Day03 {
    map: Map,
}

impl Map {
    fn new(input: &str) -> Self {
        let data: Vec<_> = input.lines().map(ToString::to_string).collect();
        let width = data[0].len();
        let height = data.len();

        Map {
            data,
            width,
            height,
        }
    }

    fn count_trees_with_slope(&self, right: usize, down: usize) -> usize {
        let mut x = 0;
        let mut y = 0;

        let mut trees = 0;
        while y < self.height {
            if let Some('#') = self.data[y].chars().nth(x) {
                trees += 1;
            }

            x = (x + right) % self.width;
            y += down;
        }

        trees
    }
}

impl Day for Day03 {
    fn number(&self) -> u8 {
        3
    }

    fn setup(&mut self, input: &str) -> Result<()> {
        self.map = Map::new(input);
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let trees = self.map.count_trees_with_slope(3, 1);
        Ok(trees.to_string())
    }

    fn part2(&self) -> Result<String> {
        let slopes = [
            self.map.count_trees_with_slope(1, 1),
            self.map.count_trees_with_slope(3, 1),
            self.map.count_trees_with_slope(5, 1),
            self.map.count_trees_with_slope(7, 1),
            self.map.count_trees_with_slope(1, 2),
        ];
        let trees: usize = slopes.iter().product();
        Ok(trees.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Day03;
    use crate::lib::advent::Day;

    #[test]
    fn example() {
        let s = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        let mut d = Day03::default();
        d.setup(s).unwrap();
        assert_eq!("7", d.part1().unwrap());
        assert_eq!("336", d.part2().unwrap());
    }
}
