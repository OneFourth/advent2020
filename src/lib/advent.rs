use std::{error::Error, path::Path};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Day {
    fn number(&self) -> u8;
    fn setup(&mut self, input: &str) -> Result<()>;
    fn part1(&self) -> Result<String>;
    fn part2(&self) -> Result<String>;

    fn run(&mut self) -> Result<()> {
        let path = Path::new(".")
            .join("input")
            .join(format!("{:02}", self.number()));
        let input = std::fs::read_to_string(path)?;

        println!("Day {:02}:", self.number());
        if input.len() > 50 {
            println!("\nInput: \"{}...\"", &input[0..50]);
        } else {
            println!("\nInput: \"{}\"", &input);
        }

        println!("\nSetup:");
        self.setup(&input)?;

        println!("\nPart 1:");
        println!("{}", self.part1()?);
        println!("\nPart 2:");
        println!("{}", self.part2()?);

        Ok(())
    }
}
