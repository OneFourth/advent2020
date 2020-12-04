use super::advent::Day;
use super::advent::Result;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn run_day(day: u8, input: &str) -> Result<()> {
    let d: Result<Box<dyn Day>> = match day {
        1 => Ok(Box::new(day01::Day01::default())),
        2 => Ok(Box::new(day02::Day02::default())),
        3 => Ok(Box::new(day03::Day03::default())),
        4 => Ok(Box::new(day04::Day04::default())),
        /*
        5 => Ok(Box::new(day05::Day05::default())),
        6 => Ok(Box::new(day06::Day06::default())),
        7 => Ok(Box::new(day07::Day07::default())),
        8 => Ok(Box::new(day08::Day08::default())),
        9 => Ok(Box::new(day09::Day09::default())),
        10 => Ok(Box::new(day10::Day10::default())),
        11 => Ok(Box::new(day11::Day11::default())),
        12 => Ok(Box::new(day12::Day12::default())),
        13 => Ok(Box::new(day13::Day13::default())),
        14 => Ok(Box::new(day14::Day14::default())),
        15 => Ok(Box::new(day15::Day15::default())),
        16 => Ok(Box::new(day16::Day16::default())),
        17 => Ok(Box::new(day17::Day17::default())),
        18 => Ok(Box::new(day18::Day18::default())),
        19 => Ok(Box::new(day19::Day19::default())),
        20 => Ok(Box::new(day20::Day20::default())),
        21 => Ok(Box::new(day21::Day21::default())),
        22 => Ok(Box::new(day22::Day22::default())),
        23 => Ok(Box::new(day23::Day23::default())),
        24 => Ok(Box::new(day24::Day24::default())),
        25 => Ok(Box::new(day25::Day25::default())),
        */
        _ => Err("Invalid Day".into()),
    };

    d?.run(input)
}
