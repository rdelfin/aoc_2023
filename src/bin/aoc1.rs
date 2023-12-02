use clap::Parser;
use color_eyre::eyre::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Parser, Debug)]
struct Opt {
    #[clap(index = 1)]
    file_name: Option<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::parse();

    let numbers = if let Some(file_name) = opt.file_name {
        let file = File::open(file_name)?;
        get_line_numbers(file)?
    } else {
        get_line_numbers(std::io::stdin())?
    };

    let sum: u32 = numbers.into_iter().sum();
    println!("{sum}");

    Ok(())
}

// This function takes a line and returns, respectively, the first and last digit, if they exist
fn get_edge_digits(line: &str) -> Option<(u32, u32)> {
    let mut digits = None;
    for c in line.chars() {
        if let Some(digit) = c.to_digit(10) {
            if let Some((_, second)) = digits.as_mut() {
                *second = digit;
            } else {
                digits = Some((digit, digit));
            }
        }
    }

    digits
}

fn get_line_numbers<R: Read>(r: R) -> std::io::Result<Vec<u32>> {
    let mut reader = BufReader::new(r);
    let mut numbers = Vec::new();

    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break;
        }

        match get_edge_digits(&line) {
            Some((l, r)) => {
                numbers.push(l * 10 + r);
            }
            _ => println!("line has no digits"),
        }
    }

    Ok(numbers)
}
