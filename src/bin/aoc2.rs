use aoc_2023::{block_game::Game, Opt};
use clap::Parser;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::parse();
    let lines = aoc_2023::read_lines(&opt)?;

    let game_ids_total: u64 = lines
        .into_iter()
        .map(|line| Game::parse(&line))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter_map(|game| {
            if game.possible(12, 13, 14) {
                println!("Game {} is possible", game.game_id);
                Some(game.game_id)
            } else {
                None
            }
        })
        .sum();

    println!("{}", game_ids_total);

    Ok(())
}
