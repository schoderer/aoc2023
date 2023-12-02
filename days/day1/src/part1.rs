pub struct Part1;

impl utils::Part for Part1 {
    type Intermediate = Option<u64>;

    type Output = u64;

    fn map(&mut self, input: &str) -> Self::Intermediate {
        let first: u64 = input
            .chars()
            .find(char::is_ascii_digit)
            .and_then(|cha| cha.to_digit(10))?
            .into();
        let last: u64 = input
            .chars()
            .rev()
            .find(char::is_ascii_digit)
            .and_then(|cha| cha.to_digit(10))?
            .into();

        Some((first * 10) + last)
    }

    fn reduce(&mut self, input: Vec<Self::Intermediate>) -> Self::Output {
        input
            .into_iter()
            .map(Option::unwrap_or_default)
            .sum::<u64>()
    }
}
#[cfg(test)]
mod tests {
    use crate::Part1;
    use utils::Part;


    #[test]
    fn sample_input(){
        let input = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;

        let mut part = Part1;

        let res = part.run_part(input);

        assert_eq!(res, 142);
    }

    #[test]
    fn main() {
        let input = include_str!("../inputs/day1.txt");

        let mut part = super::Part1;

        let result = part.run_part(input);
        assert_eq!(55130, result);
    }
}