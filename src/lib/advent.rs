use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Day<'a> {
    fn setup(&mut self, input: &'a str) -> Result<()>;
    fn part1(&self) -> Result<String>;
    fn part2(&self) -> Result<String>;

    fn run(&mut self, input: &'a str) -> Result<()> {
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
