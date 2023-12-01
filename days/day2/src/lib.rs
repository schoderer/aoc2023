pub mod parser;
mod part1;
mod part2;

pub use part1::Part1;
pub use part2::Part2;
use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub sets: Vec<Set>,
}

impl Game {
    pub fn minimal_cube_power(&self) -> usize {
        let cubes = &self
            .sets
            .iter()
            .flat_map(|set| &set.cubes)
            .collect::<Vec<_>>();
        let mut green = None;
        let mut red = None;
        let mut blue = None;

        for cube in cubes {
            match cube.color {
                Color::Red => red = red.max(Some(cube.amount)),
                Color::Blue => blue = blue.max(Some(cube.amount)),
                Color::Green => green = green.max(Some(cube.amount)),
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
            let is_invalid = match cube.color {
                Color::Red => cube.amount > max_red,
                Color::Green => cube.amount > max_green,
                Color::Blue => cube.amount > max_blue,
            };
            if is_invalid {
                return false;
            }
        }
        true
    }
}
#[derive(Debug)]
pub struct ShownCubes {
    amount: usize,
    color: Color,
}

#[derive(Debug)]
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
            _ => Err(()), // better error
        }
    }
}
