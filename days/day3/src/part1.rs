use crate::EnginePart;

pub struct Part1;

impl utils::Part for Part1 {
    type Intermediate = Vec<EnginePart>;

    type Output = u64;

    fn map(&mut self, input: &str) -> Self::Intermediate {
        super::parse(input).unwrap()
    }

    fn reduce(&mut self, _input: Vec<Self::Intermediate>) -> Self::Output {
        todo!()
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn sample_input(){
        use utils::Part;
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let mut part = super::Part1;

        let res = part.run_part(input);

        assert_eq!(res, 4361); //todo
    }

    #[test]
    fn main() {
        use utils::Part;
        let input = include_str!("../inputs/day3.txt");

        let mut part = super::Part1;

        let result = part.run_part(input);
        assert_eq!(result, 539637); //todo
    }
}