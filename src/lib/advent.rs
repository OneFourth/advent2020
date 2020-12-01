use std::{error::Error, path::Path};

pub type Result = std::result::Result<(), Box<dyn Error>>;

pub trait Day {
    fn number(&self) -> u8;
    fn setup(&mut self, input: String) -> Result;
    fn part1(&self) -> Result;
    fn part2(&self) -> Result;

    fn run(&mut self) -> Result {
        let path = Path::new(".")
            .join("input")
            .join(format!("{:02}", self.number()));
        let input = std::fs::read_to_string(path)?;

        println!("Day {:02}:", self.number());
        if input.len() > 50 {
            println!("Input: \"{}...\"", &input[0..50]);
        } else {
            println!("Input: \"{}\"", &input);
        }

        println!("Setup:");
        self.setup(input)?;

        println!("Part 1:");
        self.part1()?;
        println!("Part 2:");
        self.part2()?;

        Ok(())
    }
}
