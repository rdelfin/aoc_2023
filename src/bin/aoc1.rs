use aoc_2023::Opt;
use clap::Parser;
use color_eyre::eyre::Result;
use std::{collections::HashMap, fs::File};

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::parse();

    let digits = HashMap::from_iter(
        [
            ("0", 0u32),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]
        .into_iter(),
    );
    let reverse_digits = HashMap::from_iter(
        [
            ("0", 0u32),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("orez", 0),
            ("eno", 1),
            ("owt", 2),
            ("eerht", 3),
            ("ruof", 4),
            ("evif", 5),
            ("xis", 6),
            ("neves", 7),
            ("thgie", 8),
            ("enin", 9),
        ]
        .into_iter(),
    );

    let numbers = if let Some(file_name) = opt.file_name {
        let file = File::open(file_name)?;
        aoc_2023::get_line_numbers(file, &digits, &reverse_digits)?
    } else {
        aoc_2023::get_line_numbers(std::io::stdin(), &digits, &reverse_digits)?
    };

    let sum: u32 = numbers.into_iter().sum();
    println!("{sum}");

    Ok(())
}
