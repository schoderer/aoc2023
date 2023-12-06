mod part1;
mod part2;

pub use part1::Part1;
pub use part2::Part2;
#[derive(Debug)]
pub struct GardenEq{
    seeds: Vec<usize>,
    maps: Vec<MapTo>

}

#[derive(Debug)]
pub struct MapTo{
    from: String,
    to: String,
    values: Vec<usize>
}

pub mod parser{
    use nom::{bytes::complete::{tag, take_while}, character::complete::{digit1, line_ending, alpha1}, IResult, multi::separated_list0, combinator::{map_res, map}, sequence::tuple};

    use crate::{GardenEq, MapTo};

    pub fn parse(input: &str) -> anyhow::Result<GardenEq>{
        let res = parse_nom(input).unwrap();

        println!("{:?}", res.1);
        Ok(res.1)
    }

    pub fn parse_nom(input: &str) -> IResult<&str, GardenEq>{
        let (input, seeds) = parse_seed_list(input)?;
        let (input, seed_map) = parse_map(input)?;
        let (input, soil_map) = parse_map(input)?;
        let (input, fertilizer_map) = parse_map(input)?;
        let (input, water_map) = parse_map(input)?;
        let (input, light_map) = parse_map(input)?;
        let (input, temperature_map) = parse_map(input)?;
        let (input, humidity_map) = parse_map(input)?;

        Ok((input, GardenEq{
            seeds,
            maps: vec![seed_map, soil_map, fertilizer_map, water_map, light_map, temperature_map, humidity_map]
        }))
    }
/*

        seed-to-soil map:
        50 98 2
        52 50 48
        
*/
    pub fn parse_map(input: &str) -> IResult<&str, MapTo>{
        let (input, (_, from, _, to, _, _)) = tuple((take_spaces, alpha1, tag("-to-"), alpha1, tag(" map:"), take_spaces))(input)?;



        let map = MapTo{
            from: from.to_string(),
            to: to.to_string(),
            values: vec![]
        };
        println!("Map: {map:?}");
        Ok((input, map))
    }

    fn parse_seed_list(input: &str) -> IResult<&str, Vec<usize>> {
        map(tuple((tag("seeds:"), parse_number_list, line_ending)), |(_, nums, _)| nums)(input)
    }

    fn parse_number_list(input: &str) -> IResult<&str, Vec<usize>> {
        separated_list0(tag(" "), parse_digit)(input)
    }

    fn take_spaces(input: &str) -> IResult<&str, &str> {
        take_while(|c| c == ' ' || c == '\n')(input)
    }

    fn parse_digit(input: &str) -> IResult<&str, usize> {
        use std::str::FromStr;
        let (input, _) = take_spaces(input)?;
        map_res(digit1, usize::from_str)(input)
    }
}