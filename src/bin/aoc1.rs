use aoc_2023::Opt;
use clap::Parser;
use color_eyre::eyre::Result;
use std::fs::File;

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::parse();

    let numbers = if let Some(file_name) = opt.file_name {
        let file = File::open(file_name)?;
        aoc_2023::get_line_numbers(file)?
    } else {
        aoc_2023::get_line_numbers(std::io::stdin())?
    };

    let sum: u32 = numbers.into_iter().sum();
    println!("{sum}");

    Ok(())
}
