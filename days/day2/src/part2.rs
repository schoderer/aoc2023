use crate::{Game, parser};

pub struct Part2;

impl utils::Part for Part2 {
    type Intermediate = Game;

    type Output = usize;

    fn map(&mut self, input: &str) -> Self::Intermediate {
        parser::parse_game(input).unwrap()
    }

    fn reduce(&mut self, input: Vec<Self::Intermediate>) -> Self::Output {
        input.iter().map(Game::minimal_cube_power).sum()
    }
}

mod tests {

    #[test]
    fn sample_input(){
        use utils::Part;
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let mut part = super::Part2;

        let res = part.run_part(input);

        assert_eq!(res, 2286)
    }

    #[test]
    fn main() {
        use utils::Part;
        let input = include_str!("../inputs/day2.txt");

        let mut part = super::Part2;

        let result = part.run_part(input);
        assert_eq!(70768, result);
    }
}
