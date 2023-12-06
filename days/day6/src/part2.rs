use crate::Rounds;

pub struct Part2;

pub trait ToSingleDigit{
    fn to_single_digit(&self) -> Option<Rounds>;
}
impl ToSingleDigit for Vec<Rounds>{
    fn to_single_digit(&self) -> Option<Rounds> {
        use std::str::FromStr;
        let time = self.iter().map(|r| r.time_limit).fold(String::new(),|acc, e| format!("{acc}{e}"));
        let time: usize = usize::from_str(&time).ok()?;
        let distance = self.iter().map(|r| r.best_distance).fold(String::new(),|acc, e| format!("{acc}{e}"));
        let distance: usize = usize::from_str(&distance).ok()?;
        Some(Rounds { time_limit: time, best_distance: distance, current_round: 0 })
    }
}

impl utils::Part for Part2 {
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
        let round = input.to_single_digit().unwrap();
        
        round.into_iter().filter(|b| *b).count()
    }
}

#[cfg(test)]
mod tests {
    use crate::Part2;
    use utils::Part;


    #[test]
    fn sample_input(){
        use utils::Part;
        let input = r#"Time:      7  15   30
        Distance:  9  40  200"#;

        let mut part = super::Part2;

        let res = part.run_part(input);

        assert_eq!(res, 71503); //todo
    }

    #[test]
    fn main() {
        use utils::Part;
        let input = include_str!("../inputs/day6.txt");

        let mut part = super::Part2;

        let res = part.run_part(input);
        assert_eq!(res, 999999); //todo
    }
}