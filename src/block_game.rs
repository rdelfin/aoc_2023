use color_eyre::eyre::{eyre, Result};
use regex::Regex;

pub struct Game {
    pub game_id: u64,
    pub draws: Vec<Draw>,
}

pub struct Draw {
    pub num_red: u64,
    pub num_blue: u64,
    pub num_green: u64,
}

impl Game {
    pub fn parse(line: &str) -> Result<Game> {
        let re = Regex::new(r"^Game (\d+): (.*)")?;

        let captures = re
            .captures(line)
            .ok_or_else(|| eyre!("failed to parse line \"{line}\""))?;
        let game_id: u64 = captures
            .get(1)
            .ok_or_else(|| eyre!("failed to get capture"))?
            .as_str()
            .parse()?;
        let draws_str = captures
            .get(2)
            .ok_or_else(|| eyre!("failed to get capture"))?
            .as_str();

        let draws = draws_str
            .split(";")
            .map(|s| Draw::parse(s))
            .collect::<Result<Vec<_>>>()?;

        Ok(Game { game_id, draws })
    }

    pub fn possible(&self, red: u64, green: u64, blue: u64) -> bool {
        self.draws
            .iter()
            .all(|draw| draw.possible(red, green, blue))
    }

    pub fn min_cubes(&self) -> Draw {
        let mut min_cubes = Draw {
            num_red: 0,
            num_blue: 0,
            num_green: 0,
        };
        for draw in &self.draws {
            if draw.num_red > min_cubes.num_red {
                min_cubes.num_red = draw.num_red;
            }
            if draw.num_blue > min_cubes.num_blue {
                min_cubes.num_blue = draw.num_blue;
            }
            if draw.num_green > min_cubes.num_green {
                min_cubes.num_green = draw.num_green;
            }
        }

        min_cubes
    }
}

impl Draw {
    pub fn parse(draws_str: &str) -> Result<Draw> {
        let pair_re = Regex::new(r"^(\d+) ((blue)|(red)|(green))$")?;

        let mut num_red = 0;
        let mut num_blue = 0;
        let mut num_green = 0;
        for segment in draws_str.split(",") {
            let segment = segment.trim();
            let captures = pair_re
                .captures(segment)
                .ok_or_else(|| eyre!("failed to parse draw \"{draws_str}\""))?;
            let val = captures
                .get(1)
                .ok_or_else(|| eyre!("failed to get capture"))?
                .as_str()
                .parse()?;
            let color = captures
                .get(2)
                .ok_or_else(|| eyre!("failed to get capture"))?
                .as_str();

            match color {
                "red" => num_red = val,
                "blue" => num_blue = val,
                "green" => num_green = val,
                _ => return Err(eyre!("failed to parse color")),
            }
        }

        Ok(Draw {
            num_red,
            num_blue,
            num_green,
        })
    }

    pub fn possible(&self, red: u64, green: u64, blue: u64) -> bool {
        self.num_red <= red && self.num_blue <= blue && self.num_green <= green
    }
}
