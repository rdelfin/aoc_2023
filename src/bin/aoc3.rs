use aoc_2023::{engine::Schematic, Opt};
use clap::Parser;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::parse();
    let lines = aoc_2023::read_lines(&opt)?;

    let schematic = Schematic::parse(lines.iter().map(|s| s.as_str()));
    let part_numbers = schematic.get_valid_part_numbers();

    println!("Schematic:");
    println!("{schematic}\n");
    println!("Part numbers: {part_numbers:?}");
    println!("Sum: {}", part_numbers.iter().sum::<u64>());

    Ok(())
}
