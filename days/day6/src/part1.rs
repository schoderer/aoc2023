use crate::Rounds;

pub struct Part1;



impl utils::Part for Part1 {
    type Intermediate = Vec<Rounds>;

    type Output = usize;

    fn splitter<'a>(&mut self, input: &'a str) -> Vec<&'a str> {
        vec![input]
    }
    fn map(&mut self, input: &str) -> Self::Intermediate {
        crate::parser::parse(input)
    }

    fn reduce(&mut self, input: Vec<Self::Intermediate>) -> Self::Output {
        let input = input.into_iter().next().unwrap();
        input.into_iter().map(|round| round.into_iter().filter(|b| *b).count())
        .reduce(|acc, inc| acc * inc).unwrap()
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn sample_input(){
        use utils::Part;
        let input = r#"Time:      7  15   30
        Distance:  9  40  200"#;

        let mut part = super::Part1;

        let res = part.run_part(input);

        assert_eq!(res, 288);
    }

    #[test]
    fn main() {
        use utils::Part;
        let input = include_str!("../inputs/day6.txt");

        let mut part = super::Part1;

        let result = part.run_part(input);
        assert_eq!(800280, result); //todo
    }
}