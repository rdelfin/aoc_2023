use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

pub mod block_game;
pub mod trie;

#[derive(Parser, Debug)]
pub struct Opt {
    #[clap(index = 1)]
    pub file_name: Option<PathBuf>,
}

pub fn read_lines(opt: &Opt) -> std::io::Result<Vec<String>> {
    if let Some(file_name) = &opt.file_name {
        read_lines_impl(File::open(file_name)?)
    } else {
        read_lines_impl(std::io::stdin())
    }
}

fn read_lines_impl<R: Read>(r: R) -> std::io::Result<Vec<String>> {
    let mut reader = BufReader::new(r);
    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        lines.push(line);
    }
    Ok(lines)
}
