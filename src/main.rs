use std::error::Error;
use structopt::StructOpt;

mod lib;

use crate::lib::advent::Day;
use crate::lib::days::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "advent")]
struct Opt {
    day: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    if let Some(mut day) = get_day(opt.day) {
        day.run()?;

        Ok(())
    } else {
        Err("Invalid day".into())
    }
}
