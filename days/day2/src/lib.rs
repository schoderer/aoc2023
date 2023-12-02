mod part1;
mod part2;
mod parser;

use std::str::FromStr;
pub use part1::Part1;
pub use part2::Part2;


#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub sets: Vec<Set>,
}

impl Game {
    pub fn minimal_cube_power(&self) -> usize {
        let group = &self.sets.iter()
            .flat_map(|set| &set.cubes)
            .collect::<Vec<_>>()
            ;
        let mut green = None;
        let mut red = None;
        let mut blue = None;

        for (amount, color) in group{
            match color {
                Color::Red => red = red.max(Some(*amount)),
                Color::Blue => blue = blue.max(Some(*amount)),
                Color::Green => green = green.max(Some(*amount)),
            }

        }


        green.unwrap_or_default() * red.unwrap_or_default() * blue.unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct Set {
    pub cubes: Vec<ShownCubes>,
}

impl Set {
    fn is_valid_for(&self, max_red: usize, max_green: usize, max_blue: usize) -> bool {
        for cube in &self.cubes {
            let is_invalid = match cube.1 {
                Color::Red => cube.0 > max_red,
                Color::Green => cube.0 > max_green,
                Color::Blue => cube.0 > max_blue,
            };
            if is_invalid {
                return false;
            }
        }
        true
    }
}

type ShownCubes = (usize, Color);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            _ => Err(()) // better error
        }
    }
}