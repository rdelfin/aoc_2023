use clap::Parser;
use std::io::{BufRead, BufReader, Read};

#[derive(Parser, Debug)]
pub struct Opt {
    #[clap(index = 1)]
    pub file_name: Option<String>,
}

// This function takes a line and returns, respectively, the first and last digit, if they exist
pub fn get_edge_digits(line: &str) -> Option<(u32, u32)> {
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

pub fn get_line_numbers<R: Read>(r: R) -> std::io::Result<Vec<u32>> {
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
