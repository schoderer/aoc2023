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
        let wins = self.wins();
        for _ in 1..=wins{
            if score == 0{
                score = 1;
            }else {
                score *= 2;
            }

        }
        score
    }

    pub fn wins(&self) -> usize {
        self.winning_numbers.iter()
            .filter(|num| self.own_numbers.contains(*num))
            .count()
    }
}

impl FromStr for Card{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, card) = parser::parse(s).unwrap();
        Ok(card)
    }
}

mod parser {
    use std::str::FromStr;
    use nom::bytes::complete::{tag, take_while};
    use nom::character::complete::{digit1};
    use nom::combinator::{map, map_res};
    use nom::{IResult};
    use nom::multi::{separated_list0};
    use nom::sequence::tuple;
    use crate::Card;

    pub fn parse(input: &str) -> nom::IResult<&str, Card>{
        let (input, number) = parse_card_number(input)?;
        let (input, _) = take_spaces(input)?;
        let (input, winning_numbers) = parse_number_list(input)?;
        let (input, _) = tag(" | ")(input)?;
        let (input, _) = take_while(|c| c == ' ')(input)?;
        let (input, own_numbers) = parse_number_list(input)?;
        Ok((input, Card{
            number,
            winning_numbers,
            own_numbers
        }))
    }

    fn parse_card_number(input: &str) -> IResult<&str, usize>{
        map(tuple((tag("Card"), take_spaces, map_res(digit1, usize::from_str), tag(":"))), |(_, _, num, _)| num)(input)
    }

    fn parse_number_list(input: &str) -> IResult<&str, Vec<usize>>{
        separated_list0(tag(" "), parse_digit)(input)
    }

    fn take_spaces(input: &str) -> IResult<&str, &str>{
        take_while(|c| c == ' ')(input)
    }

    fn parse_digit(input: &str) -> IResult<&str, usize>{
        let (input, _) = take_spaces(input)?;
        map_res(digit1, usize::from_str)(input)
    }


}
