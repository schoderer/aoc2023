mod part1;
mod part2;

pub use part1::Part1;
pub use part2::Part2;

#[derive(Debug)]
pub struct Rounds{
    time_limit: usize,
    best_distance: usize,
    current_round: usize,
}

impl Iterator for Rounds{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_round += 1;
        if self.current_round > self.time_limit {
            return None;
        }
        let distance = calc_distance(self.current_round, self.time_limit);
        Some(distance > self.best_distance)
        
    }


}

fn calc_distance(charge_time: usize, max_time: usize) -> usize{
    let remaining = max_time - charge_time;
    remaining * charge_time
}

pub mod parser{
    use nom::{bytes::complete::{tag, take_while}, character::complete::{line_ending, digit1}, IResult, sequence::tuple, multi::separated_list0, combinator::map_res};

    use crate::Rounds;

    pub fn parse(input: &str) -> Vec<Rounds>{

        let (_,res) = parse_nom(input).unwrap();
        res
    }

    fn parse_nom(input: &str) -> IResult<&str, Vec<Rounds>>{
        let (input, (_, times, _)) = tuple((tag("Time:"), parse_number_list, line_ending))(input)?;
        let (input, (_,_, distance)) = tuple((take_spaces, tag("Distance:"), parse_number_list))(input)?;
        let res: Vec<Rounds> = times.iter().zip(distance.iter())
        .map(|(time_limit, best_distance)|Rounds{best_distance: *best_distance, time_limit: *time_limit, current_round: 0})
        .collect();
        println!("{res:?}");
        Ok((input, res))
    }

    fn parse_number_list(input: &str) -> IResult<&str, Vec<usize>> {
        separated_list0(tag(" "), parse_digit)(input)
    }

    fn take_spaces(input: &str) -> IResult<&str, &str> {
        take_while(|c| c == ' ')(input)
    }

    fn parse_digit(input: &str) -> IResult<&str, usize> {
        use std::str::FromStr;
        let (input, _) = take_spaces(input)?;
        map_res(digit1, usize::from_str)(input)
    }
}