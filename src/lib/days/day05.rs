use std::convert::TryInto;

use crate::lib::advent::Result;
use crate::Day;

struct Seat<'a> {
    ins: &'a [u8; 10],
}

#[derive(Default)]
pub struct Day05 {
    seat_ids: Vec<usize>,
}

impl<'a> Seat<'a> {
    fn new(ins: &'a [u8; 10]) -> Self {
        Self { ins }
    }

    fn get_seat_id(&self) -> usize {
        const BASE: usize = 2;
        self.ins
            .iter()
            .rev()
            .enumerate()
            .map(|(n, &r)| BASE.pow(n as u32) * matches!(r, b'R' | b'B') as usize)
            .sum()
    }
}

impl<'a> Day<'a> for Day05 {
    fn setup(&mut self, input: &'a str) -> Result<()> {
        self.seat_ids = input
            .lines()
            .map(|instructions| {
                let b = instructions.as_bytes().try_into()?;
                Ok(Seat::new(b).get_seat_id())
            })
            .collect::<Result<Vec<_>>>()?;
        self.seat_ids.sort();
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        match self.seat_ids.as_slice() {
            [.., max] => Ok(max.to_string()),
            _ => Err("No seats".into()),
        }
    }

    fn part2(&self) -> Result<String> {
        match self.seat_ids.as_slice() {
            [min, .., max] => Ok(self
                .seat_ids
                .iter()
                .zip(*min..*max)
                .filter_map(|(&s, v)| if s != v { Some(s - 1) } else { None })
                .next()
                .ok_or("No seat found")?
                .to_string()),
            _ => Err("No seats".into()),
        }
    }
}
