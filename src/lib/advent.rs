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

        let mut total = 0;

        {
            let before = std::time::Instant::now();

            println!("\nSetup:");
            self.setup(&input)?;

            let after = std::time::Instant::now();
            let duration = (after - before).as_millis();
            total += duration;
            println!("\nTook: {}ms", duration);
        }

        {
            let before = std::time::Instant::now();

            println!("\nPart 1:");
            println!("{}", self.part1()?);

            let after = std::time::Instant::now();
            let duration = (after - before).as_millis();
            total += duration;
            println!("\nTook: {}ms", duration);
        }

        {
            let before = std::time::Instant::now();

            println!("\nPart 2:");
            println!("{}", self.part2()?);

            let after = std::time::Instant::now();
            let duration = (after - before).as_millis();
            total += duration;
            println!("\nTook: {}ms", duration);
        }

        println!("\nTotal time: {}ms", total);

        Ok(())
    }
}
