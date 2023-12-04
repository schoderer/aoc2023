mod part1;
mod part2;

use std::str::FromStr;
pub use part1::Part1;
pub use part2::Part2;


#[derive(Debug)]
pub struct Card{
    number: usize,
    winning_numbers: Vec<usize>,
    own_numbers: Vec<usize>
}

impl Card{
    pub fn score(&self) -> usize{
        let mut score = 0;
        for _ in 0..=self.wins(){
            if score == 0{
                score = 1;
            }else {
                score *= 2;
            }

        }
        score
    }

    fn wins(&self) -> usize {
        self.own_numbers.iter()
            .filter(|num| self.winning_numbers.contains(*num))
            .count()
    }
}

impl FromStr for Card{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::parse(s)
    }
}

mod parser {
    use std::str::FromStr;
    use nom::bytes::complete::{tag, take_while};
    use nom::character::complete::digit1;
    use nom::combinator::{map, map_res};
    use nom::{Finish, IResult};
    use nom::multi::{separated_list0, separated_list1};
    use nom::sequence::tuple;
    use crate::Card;

    pub fn parse(input: &str) -> anyhow::Result<Card>{

        let card_parser = tuple((parse_card_number, parse_number_list, take_spaces, tag("|"), take_spaces, parse_number_list));
        let mut card_mapper = map(card_parser, |(number, winning_numbers, _, _, _, own_numbers)| Card{number, winning_numbers, own_numbers});
        let (_, card) = card_mapper(input)
            .finish()
            .map_err(|err| nom::error::Error::new(err.input.to_string(), err.code))?;
        Ok(card)

    }

    fn parse_card_number(input: &str) -> IResult<&str, usize>{
        map_res(tuple((tag("Card"), take_spaces, digit1, tag(": "))), |(_, _, num, _)|  usize::from_str(num))(input)
    }

    fn parse_number_list(input: &str) -> IResult<&str, Vec<usize>>{
        separated_list0(take_spaces, parse_digit)(input)
    }

    fn take_spaces(input: &str) -> IResult<&str, &str>{
        take_while(|c| c == ' ')(input)
    }

    fn parse_digit(input: &str) -> IResult<&str, usize>{
        map_res(digit1, usize::from_str)(input)
    }


}
