use std::str::FromStr;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{char};
use nom::combinator::{all_consuming, map, map_res};
use nom::IResult;
use nom::multi::{separated_list1};
use nom::sequence::tuple;
use crate::{Color, Game, Set, ShownCubes};

pub fn parse_game(input: &str) -> Result<Game, std::io::Error> {
    let game_parser = tuple((tag("Game "), parse_number, tag(": "), parse_sets));
    let game_parser = map(game_parser, |(_, id, _, sets)| Game { id, sets });
    let (_, game) = all_consuming(game_parser)(input).unwrap(); //todo real nom error conversion
    Ok(game)
}


/// Takes the input an parses them to sets, until ';' or line ending
/// Input: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn parse_sets(input: &str) -> IResult<&str, Vec<Set>> {
    separated_list1(tag("; "), parse_set)(input)
}

/// Parses a single set of a game
fn parse_set(input: &str) -> IResult<&str, Set> {
    map(separated_list1(tag(", "), parse_to_shown_cubes), |cubes| Set { cubes })(input)
}

fn parse_to_shown_cubes(input: &str) -> IResult<&str, ShownCubes> {
    let map_to_cubes = |(number, _, color)| (number, color);
    map(tuple((parse_number, char(' '), parse_color)), map_to_cubes)(input)
}


fn parse_color(input: &str) -> IResult<&str, Color> {
    map_res(alt((tag("red"), tag("blue"), tag("green"))), Color::from_str)(input)
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(take_while(is_digit), usize::from_str)(input)
}


fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use crate::{parser};

    #[test]
    fn parse_single_line() {
        let input = "Game 23: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let game = parser::parse_game(input).unwrap();

        assert_eq!(game.id, 23);
        assert_eq!(game.sets.len(), 3);
    }
}