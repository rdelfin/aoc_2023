use crate::trie::Trie;
use clap::Parser;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

mod trie;

#[derive(Parser, Debug)]
pub struct Opt {
    #[clap(index = 1)]
    pub file_name: Option<String>,
}

// This function takes a line and returns, respectively, the first and last digit, if they exist
pub fn get_first_digit<I: Iterator<Item = char> + Clone>(
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
