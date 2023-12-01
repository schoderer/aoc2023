pub struct Part2;

impl utils::Part for Part2 {
    type Intermediate = ();

    type Output = u64;

    fn map(&mut self, _input: &str) -> Self::Intermediate {
        todo!()
    }

    fn reduce(&mut self, _input: Vec<Self::Intermediate>) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Part2;
    use utils::Part;


    #[test]
    fn sample_input(){
        use utils::Part;
        let input = r#"
        "#;

        let mut part = super::Part2;

        let res = part.run_part(input);

        assert_eq!(res, 999999); //todo
    }

    #[test]
    fn main() {
        use utils::Part;
        let input = include_str!("../inputs/{{project-name}}.txt");

        let mut part = super::Part2;

        let result = part.run_part(input);
        assert_eq!(res, 999999); //todo
    }
}