use aoc_2023::{trie::Trie, Opt};
use clap::Parser;
use color_eyre::eyre::Result;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

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
        get_line_numbers(file, &digits, &reverse_digits)?
    } else {
        get_line_numbers(std::io::stdin(), &digits, &reverse_digits)?
    };

    let sum: u32 = numbers.into_iter().sum();
    println!("{sum}");

    Ok(())
}

// This function takes a line and returns, respectively, the first and last digit, if they exist
fn get_first_digit<I: Iterator<Item = char> + Clone>(
    line_iter: I,
    digit_map: &HashMap<&'static str, u32>,
    trie: &Trie,
) -> Option<u32> {
    let mut result = None;
    let mut searcher = trie.get_searcher();

    let mut char_iter = line_iter.clone();
    let mut last_word_iter = line_iter.clone();
    while let Some(c) = char_iter.next() {
        match searcher.advance(c) {
            Ok(Some(val)) => {
                for _ in 0..searcher.len() {
                    last_word_iter.next().expect("should match char_iter");
                }
                let digit = digit_map
                    .get(&val[..])
                    .expect("must exist due to construction");
                result = Some(*digit);
                break;
            }
            Err(_) => {
                // If we get no matches, we need to go all the way back to the first character we
                // processed and skip just that one
                last_word_iter
                    .next()
                    .expect("next should exist as we've seen a word before");
                char_iter = last_word_iter.clone();
                searcher = trie.get_searcher();
            }
            _ => {}
        }
    }

    result
}

// Returns the first and last digit all the lines provided
pub fn get_line_numbers<R: Read>(
    r: R,
    fwd_digit_map: &HashMap<&'static str, u32>,
    bwd_digit_map: &HashMap<&'static str, u32>,
) -> std::io::Result<Vec<u32>> {
    let fwd_trie = get_trie(fwd_digit_map);
    let bwd_trie = get_trie(bwd_digit_map);

    let mut reader = BufReader::new(r);
    let mut numbers = Vec::new();

    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break;
        }

        let first_digit = if let Some(d) = get_first_digit(line.chars(), &fwd_digit_map, &fwd_trie)
        {
            d
        } else {
            println!("line has no digits");
            continue;
        };
        let last_digit =
            if let Some(d) = get_first_digit(line.chars().rev(), &bwd_digit_map, &bwd_trie) {
                d
            } else {
                println!("line has no digits");
                continue;
            };

        numbers.push(first_digit * 10 + last_digit);
    }

    Ok(numbers)
}

fn get_trie(digit_map: &HashMap<&'static str, u32>) -> Trie {
    Trie::new(digit_map.iter().map(|(s, _)| *s))
}
