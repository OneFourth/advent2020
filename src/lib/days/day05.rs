use std::convert::TryInto;

use crate::lib::advent::Result;
use crate::Day;

struct Seat<'a> {
    row_ins: &'a [u8],
    col_ins: &'a [u8],
}

#[derive(Default)]
pub struct Day05<'a> {
    seats: Vec<Seat<'a>>,
}

impl<'a> Seat<'a> {
    fn new(s: &'a [u8; 10]) -> Self {
        let (row_ins, col_ins) = s.split_at(7);

        Self { row_ins, col_ins }
    }

    fn get_value(ins: &[u8], one: u8) -> usize {
        const BASE: usize = 2;
        ins.iter()
            .rev()
            .enumerate()
            .map(|(n, &r)| BASE.pow(n as u32) * (r == one) as usize)
            .sum()
    }

    fn get_seat_id(&self) -> usize {
        let r = Seat::get_value(self.row_ins, b'B');
        let c = Seat::get_value(self.col_ins, b'R');
        r * 8 + c
    }
}

impl<'a> Day<'a> for Day05<'a> {
    fn setup(&mut self, input: &'a str) -> Result<()> {
        self.seats = input
            .lines()
            .map(|instructions| {
                let b = instructions.as_bytes().try_into()?;
                Ok(Seat::new(b))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let max = self
            .seats
            .iter()
            .map(|s| s.get_seat_id())
            .max()
            .ok_or("Couldn't find max")?;
        Ok(max.to_string())
    }

    fn part2(&self) -> Result<String> {
        let mut seat_ids: Vec<_> = self.seats.iter().map(|s| s.get_seat_id()).collect();
        seat_ids.sort();
        let min = seat_ids.get(0).ok_or("No seat IDs")?;
        let max = seat_ids.iter().last().ok_or("No seat IDs")?;

        let seat = seat_ids
            .iter()
            .zip(*min..*max)
            .filter_map(|(&s, v)| if s != v { Some(s - 1) } else { None })
            .next()
            .ok_or("No seat found")?;

        Ok(seat.to_string())
    }
}
