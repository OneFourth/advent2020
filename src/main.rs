use std::path::Path;
use structopt::StructOpt;

mod lib;

use crate::lib::advent::Day;
use crate::lib::advent::Result;
use crate::lib::days::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "advent")]
struct Opt {
    day: u8,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    println!("Day {:02}:", opt.day);

    let path = Path::new(".").join("input").join(format!("{:02}", opt.day));
    let input = std::fs::read_to_string(path)?;

    run_day(opt.day, &input)
}
