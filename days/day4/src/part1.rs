use crate::Card;
use std::str::FromStr;

pub struct Part1;

impl utils::Part for Part1 {
    type Intermediate = Card;
    type Output = usize;

    fn map(&mut self, input: &str) -> Self::Intermediate {
        Card::from_str(input).unwrap()
    }

    fn reduce(&mut self, input: Vec<Self::Intermediate>) -> Self::Output {
        input.iter().map(Card::score).sum()
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn sample_input() {
        use utils::Part;
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14 1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let mut part = super::Part1;

        let res = part.run_part(input);

        assert_eq!(res, 13);
    }
    #[test]
    fn sample_input1() {
        use utils::Part;
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"#;

        let mut part = super::Part1;

        let res = part.run_part(input);

        assert_eq!(res, 8);
    }

    #[test]
    fn main() {
        use utils::Part;
        let input = include_str!("../inputs/day4.txt");

        let mut part = super::Part1;

        let result = part.run_part(input);
        assert_eq!(33950, result);
    }
}
