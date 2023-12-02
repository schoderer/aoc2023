pub struct Part1;

impl utils::Part for Part1 {
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
    use crate::Part1;
    use utils::Part;


    #[test]
    fn sample_input(){
        let input = r#"
        "#;

        let mut part = Part1;

        let res = part.run_part(input);

        assert_eq!(res, 9999); //todo
    }

    #[test]
    fn main() {
        let input = include_str!("../inputs/{{project-name}}.txt");

        let mut part = Part1;

        let result = part.run_part(input);
        assert_eq!(9999, result); //todo
    }
}