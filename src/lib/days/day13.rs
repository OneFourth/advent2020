use crate::lib::advent::Result;
use crate::Day;

#[derive(Default)]
pub struct Day13<'a> {
    earliest: usize,
    times: Vec<&'a str>,
}

fn mod_inv(a: isize, module: isize) -> isize {
    let mut mn = (module, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    while xy.0 < 0 {
        xy.0 += module;
    }
    xy.0
}

impl<'a> Day<'a> for Day13<'a> {
    fn setup(&mut self, input: &'a str) -> Result<()> {
        let mut lines = input.lines();
        self.earliest = lines.next().ok_or("Empty input")?.parse()?;
        self.times = lines.next().ok_or("No times")?.split(',').collect();

        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let result = self
            .times
            .iter()
            .filter_map(|s| s.parse::<usize>().ok())
            .map(|v| {
                let t = self.earliest / v;
                if v * t < self.earliest {
                    (v, (t + 1) * v)
                } else {
                    (v, t * v)
                }
            })
            .min_by_key(|(_, time)| *time)
            .map(|(v, time)| v * (time - self.earliest))
            .ok_or("Could not find value")?;

        Ok(result.to_string())
    }

    fn part2(&self) -> Result<String> {
        let times: Vec<_> = self
            .times
            .iter()
            .enumerate()
            .filter_map(|(n, s)| s.parse::<usize>().ok().map(|v| ((v - (n % v)) % v, v)))
            .collect();

        let all_m: usize = times.iter().map(|(_, m)| m).product();

        let result = times
            .iter()
            .map(|(a, m)| {
                let bi = all_m / m;
                let bip = mod_inv(bi as isize, *m as isize) as usize;
                let r = (bi * bip) % all_m;
                a * r
            })
            .sum::<usize>();
        let result = result % all_m;

        Ok(result.to_string())
    }
}
