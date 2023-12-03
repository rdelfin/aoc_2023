use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    iter::Iterator,
};

lazy_static! {
    static ref SYMBOLS: HashSet<char> = {
        [
            '!', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '/', ':', ';', '<', '=',
            '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
        ]
        .into_iter()
        .collect()
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schematic {
    digits: HashMap<Vec2, Digit>,
    symbols: HashMap<Vec2, Symbol>,
    grid: Vec<Vec<char>>,
    longest_digit: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    pub fn new(x: i64, y: i64) -> Vec2 {
        Vec2 { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gear {
    pub pos: Vec2,
    pub parts: (u64, u64),
}

impl Gear {
    pub fn ratio(&self) -> u64 {
        self.parts.0 * self.parts.1
    }
}

impl Schematic {
    pub fn parse<'a, I: Iterator<Item = &'a str>>(lines: I) -> Schematic {
        let mut digits = HashMap::new();
        let mut symbols = HashMap::new();
        let mut grid = Vec::new();

        for (y, line) in lines.enumerate() {
            grid.push(line.chars().collect::<Vec<_>>());
            let (new_digits, new_symbols) = Self::parse_line(line, y as u64);
            digits.extend(new_digits.into_iter());
            symbols.extend(new_symbols.into_iter());
        }

        let longest_digit = digits.iter().map(|(_, d)| d.len()).max().unwrap_or(0) as u64;

        Schematic {
            digits,
            symbols,
            grid,
            longest_digit,
        }
    }

    pub fn parse_line(line: &str, y: u64) -> (Vec<(Vec2, Digit)>, Vec<(Vec2, Symbol)>) {
        let y = y as i64;

        let mut digits = Vec::new();
        let mut symbols = Vec::new();

        let mut curr_digit = None;

        for (x, c) in line.chars().enumerate() {
            let x = x as i64;
            if let Some(d) = c.to_digit(10) {
                if let Some(curr) = &mut curr_digit {
                    *curr = *curr * 10 + d;
                } else {
                    curr_digit = Some(d);
                }
            } else {
                if let Some(curr) = curr_digit {
                    let digit = Digit(curr as u64);
                    digits.push((Vec2::new(x - digit.len() as i64, y), digit));
                    curr_digit = None;
                }

                if SYMBOLS.contains(&c) {
                    symbols.push((Vec2::new(x, y), Symbol(c)));
                }
            }
        }

        // In case a digit is at the end of the line
        if let Some(curr) = curr_digit {
            let digit = Digit(curr as u64);
            let x_idx = line.len() as i64 - digit.len() as i64;
            digits.push((Vec2::new(x_idx, y), digit));
        }

        (digits, symbols)
    }

    pub fn get_valid_part_numbers(&self) -> Vec<u64> {
        self.digits
            .iter()
            .filter_map(|(pos, digit)| {
                if self.has_symbol_around(*pos, digit) {
                    Some(digit.0)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_gears(&self) -> Vec<Gear> {
        self.symbols
            .iter()
            .filter_map(|(pos, symbol)| {
                let pos = *pos;
                if symbol.0 == '*' {
                    self.get_gear_parts(pos).map(|parts| Gear { pos, parts })
                } else {
                    None
                }
            })
            .collect()
    }

    fn has_symbol_around(&self, pos: Vec2, digit: &Digit) -> bool {
        // We need to check:
        // - Blocks to the left and right
        // - Blocks on 4 corners
        // - The 1 or more blocks above and below
        // Each of these are in their own category

        let digit_len = digit.len() as i64;

        // left and right
        let left = self.has_symbol(pos + Vec2::new(-1, 0));
        let right = self.has_symbol(pos + Vec2::new(digit_len, 0));

        // Four corners
        let top_left = self.has_symbol(pos + Vec2::new(-1, -1));
        let top_right = self.has_symbol(pos + Vec2::new(digit_len, -1));
        let bottom_left = self.has_symbol(pos + Vec2::new(-1, 1));
        let bottom_right = self.has_symbol(pos + Vec2::new(digit_len, 1));

        // Top and bottom
        let top = (0..digit_len).any(|x| self.has_symbol(pos + Vec2::new(x, -1)));
        let bottom = (0..digit_len).any(|x| self.has_symbol(pos + Vec2::new(x, 1)));

        left || right || top_left || top_right || bottom_left || bottom_right || top || bottom
    }

    fn has_symbol(&self, pos: Vec2) -> bool {
        if pos.x < 0
            || pos.y < 0
            || pos.y >= self.grid.len() as i64
            || pos.x >= self.grid[0].len() as i64
        {
            return false;
        }

        SYMBOLS.contains(&self.grid[pos.y as usize][pos.x as usize])
    }

    fn get_gear_parts(&self, pos: Vec2) -> Option<(u64, u64)> {
        // The row above and below, we need to check starting from the left corner - the longest
        // digit, and confirm if any are within the range of this gear
        let mut part_values = (None, None);

        for y in -1..=1 {
            let mut x = -(self.longest_digit as i64);
            while x <= 1 {
                if x == 0 && y == 0 {
                    x += 1;
                    continue;
                }

                let curr = Vec2::new(pos.x + x, pos.y + y);
                if let Some(digit) = self.digits.get(&curr) {
                    // If this is true, it MUST be touching the gear
                    if curr.x + digit.len() as i64 >= pos.x {
                        // Skip over the digit
                        x += digit.len() as i64;

                        // Add it to the part values
                        if let (None, None) = part_values {
                            part_values.0 = Some(digit.0);
                        } else if let (Some(_), None) = part_values {
                            part_values.1 = Some(digit.0);
                        } else {
                            return None;
                        }
                    } else {
                        x += 1;
                    }
                } else {
                    x += 1;
                }
            }
        }

        if let (Some(a), Some(b)) = part_values {
            Some((a, b))
        } else {
            None
        }
    }
}

impl std::fmt::Display for Schematic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let digits = self.digits.iter().map(|(v, digit)| format!("{v}: {digit}"));
        let symbols: Vec<_> = self
            .symbols
            .iter()
            .map(|(v, symbol)| format!("{v}: {symbol}"))
            .collect();

        writeln!(f, "\tDigits:")?;
        for digit in digits {
            writeln!(f, "\t\t{}", digit)?;
        }
        writeln!(f, "\tSymbols:")?;
        for (idx, symbol) in symbols.iter().enumerate() {
            if idx == symbols.len() - 1 {
                write!(f, "\t\t{}", symbol)?;
            } else {
                writeln!(f, "\t\t{}", symbol)?;
            }
        }
        Ok(())
    }
}

trait Block {
    fn len(&self) -> u64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Digit(pub u64);

impl std::fmt::Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symbol(pub char);

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Block for Digit {
    fn len(&self) -> u64 {
        self.0.ilog10() as u64 + 1
    }
}

impl Block for Symbol {
    fn len(&self) -> u64 {
        1
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
