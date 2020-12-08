use std::{collections::HashSet, str::FromStr};

use crate::lib::advent::Result;
use crate::Day;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<_>>();
        match split.as_slice() {
            [ins, v] => match *ins {
                "acc" => Ok(Operation::Acc(v.parse().unwrap())),
                "nop" => Ok(Operation::Nop(v.parse().unwrap())),
                "jmp" => Ok(Operation::Jmp(v.parse().unwrap())),
                _ => Err("Failed to parse instruction"),
            },
            _ => Err("Invalid instruction"),
        }
    }
}

#[derive(Default, Clone)]
struct Program {
    instructions: Vec<Operation>,
    acc: isize,
    counter: isize,
}

impl Program {
    fn new(s: &str) -> Result<Self> {
        let instructions = s
            .lines()
            .map(|s| s.parse().map_err(|e: &'static str| e.into()))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            instructions,
            acc: 0,
            counter: 0,
        })
    }

    fn step(&mut self) -> bool {
        let ins = self.instructions.get(self.counter as usize);

        match ins {
            Some(Operation::Acc(inc)) => self.acc += inc,
            Some(Operation::Jmp(jump)) => {
                self.counter += jump;
                return true;
            }
            Some(Operation::Nop(_)) => {}
            None => return false,
        }

        self.counter += 1;
        true
    }

    fn get_acc_value(&mut self) -> isize {
        let mut ran = HashSet::new();
        loop {
            if !ran.insert(self.counter) {
                break;
            }
            if !self.step() {
                break;
            }
        }
        self.acc
    }
}

#[derive(Default)]
pub struct Day08 {
    program: Program,
}

impl<'a> Day<'a> for Day08 {
    fn setup(&mut self, input: &'a str) -> Result<()> {
        self.program = Program::new(input)?;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(self.program.clone().get_acc_value().to_string())
    }

    fn part2(&self) -> Result<String> {
        let mut tried_up_to = 0;
        let mut result;
        loop {
            let mut program = self.program.clone();
            let (n, ins) = program
                .instructions
                .iter_mut()
                .enumerate()
                .skip(tried_up_to)
                .filter(|(_, i)| matches!(i, Operation::Jmp(_) | Operation::Nop(_)))
                .next().unwrap();

            tried_up_to = n+1;

            match ins {
                Operation::Nop(v) => {*ins = Operation::Jmp(*v)},
                Operation::Jmp(v) => {*ins = Operation::Nop(*v)},
                _ => {}
            }

            result = program.get_acc_value();
            if program.counter as usize >= program.instructions.len() {
                break;
            }
        }
        Ok(result.to_string())
    }
}
