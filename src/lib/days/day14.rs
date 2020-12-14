use std::collections::HashMap;

use crate::lib::{advent::Result, utils::between};
use crate::Day;

#[derive(Debug, Copy, Clone)]
enum Instruction<'a> {
    SetMask(&'a str),
    SetValue(usize, usize),
}

#[derive(Debug, Default, Clone)]
struct Program<'a> {
    instructions: Vec<Instruction<'a>>,
    memory: HashMap<usize, usize>,
    mask: &'a str,
}

impl<'a> Program<'a> {
    fn apply_mask_part1(mask: &'a str, mut val: usize) -> usize {
        for (n, b) in mask.chars().rev().enumerate() {
            match b {
                '0' => val &= !(1 << n),
                '1' => val |= 1 << n,
                _ => {}
            }
        }
        val
    }

    fn run_part1(&mut self) {
        for i in &self.instructions {
            match i {
                Instruction::SetMask(m) => self.mask = m,
                Instruction::SetValue(address, val) => {
                    let v = self.memory.entry(*address).or_insert(0);
                    *v = *val;
                    *v = Self::apply_mask_part1(self.mask, *v);
                }
            }
        }
    }

    fn apply_mask_part2(mask: &'a str, mut add: usize) -> Vec<usize> {
        let mut floating = Vec::new();
        for (n, b) in mask.chars().rev().enumerate() {
            match b {
                'X' => floating.push(n),
                '1' => add |= 1 << n,
                _ => {}
            }
        }

        let n = floating.len();
        (0..2u32.pow(n as u32))
            .map(|v| {
                let mut new_add = add;
                for i in 0..n {
                    if v & (1 << i) != 0 {
                        new_add |= 1 << floating[i];
                    } else {
                        new_add &= !(1 << floating[i]);
                    }
                }
                new_add
            })
            .collect()
    }

    fn run_part2(&mut self) {
        for i in &self.instructions {
            match i {
                Instruction::SetMask(m) => self.mask = m,
                Instruction::SetValue(address, val) => {
                    for add in Self::apply_mask_part2(self.mask, *address) {
                        *self.memory.entry(add).or_insert(0) = *val;
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct Day14<'a> {
    program: Program<'a>,
}

impl<'a> Day<'a> for Day14<'a> {
    fn setup(&mut self, input: &'a str) -> Result<()> {
        for l in input.lines() {
            let ins: Vec<_> = l.split(" = ").collect();
            match ins.as_slice() {
                ["mask", m] => self.program.instructions.push(Instruction::SetMask(m)),
                [mem, val] => {
                    let address = between(mem, '[', ']')?;
                    self.program
                        .instructions
                        .push(Instruction::SetValue(address.parse()?, val.parse()?));
                }
                _ => return Err("Invalid instruction".into()),
            }
        }
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let mut p = self.program.clone();
        p.run_part1();
        let result: usize = p.memory.values().sum();
        Ok(result.to_string())
    }

    fn part2(&self) -> Result<String> {
        let mut p = self.program.clone();
        p.run_part2();
        let result: usize = p.memory.values().sum();
        Ok(result.to_string())
    }
}
