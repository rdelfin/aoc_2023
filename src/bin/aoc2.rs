use aoc_2023::{block_game::Game, Opt};
use clap::Parser;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::parse();
    let lines = aoc_2023::read_lines(&opt)?;

    let games = lines
        .into_iter()
        .map(|line| Game::parse(&line))
        .collect::<Result<Vec<_>>>()?;

    let game_ids_total: u64 = games
        .iter()
        .filter_map(|game| {
            if game.possible(12, 13, 14) {
                println!("Game {} is possible", game.game_id);
                Some(game.game_id)
            } else {
                None
            }
        })
        .sum();

    let power_sum: u64 = games
        .iter()
        .map(|game| {
            let min = game.min_cubes();
            min.num_red * min.num_blue * min.num_green
        })
        .sum();

    println!("Valid game ID sum: {}", game_ids_total);
    println!("Game power sum: {}", power_sum);

    Ok(())
}
