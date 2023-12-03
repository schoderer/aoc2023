mod part1;
mod part2;

pub use part1::Part1;
pub use part2::Part2;
pub(crate) use parser::parse;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum EnginePart {
    Symbol,
    Number(u64),
    NumberContinue,
    Empty
}



mod parser{
    use std::str::FromStr;
    use nom::branch::alt;
    use nom::character::complete::{char, digit1, satisfy};
    use nom::combinator::{all_consuming, map, map_res};
    use nom::Finish;
    use nom::multi::many1;
    use crate::EnginePart;

    pub fn parse(line: &str) -> anyhow::Result<Vec<EnginePart>>{
        let (_, result) = all_consuming(many1(alt((parse_empty, parse_digit, parse_symbol))))(line)
            .finish().map_err(|err| nom::error::Error::new(err.input.to_string(), err.code))?;
        Ok(result)
    }

    fn parse_empty(input: &str) -> nom::IResult<&str, EnginePart>{
        map(char('.'), |_| EnginePart::Empty)(input)
    }
    fn parse_symbol(input: &str) -> nom::IResult<&str, EnginePart>{
        map(satisfy(|c| c!='.' && !c.is_ascii_digit()), |_c| EnginePart::Symbol)(input)
    }
    fn parse_digit(input: &str) -> nom::IResult<&str, EnginePart>{
        map(map_res(digit1, u64::from_str), EnginePart::Number)(input)
    }
}