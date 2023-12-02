use crate::*;

pub struct Part1;

impl utils::Part for Part1 {
    type Intermediate = Game;

    type Output = usize;

    fn map(&mut self, input: &str) -> Self::Intermediate {
        parser::parse_game(input).unwrap()
    }

    fn reduce(&mut self, input: Vec<Self::Intermediate>) -> Self::Output {
        input.into_iter().filter(is_game_possible).map(|game| game.id).sum()
    }
}

fn is_game_possible(game: &Game) -> bool{
     game.sets.iter().find(|set| !set.is_valid_for(12, 13, 14)).is_none()

}
#[cfg(test)]
mod tests {
    use crate::Part1;
    use utils::Part;


    #[test]
    fn sample_input(){
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
Game 6: 6 red, 1 blue, 14 green; 2 blue, 1 red, 12 green"#;

        let mut part = Part1;

        let res = part.run_part(input);

        assert_eq!(res, 14)
    }

    #[test]
    fn main() {
        let input = include_str!("../inputs/day02.txt");

        let mut part = super::Part1;

        let result = part.run_part(input);
        assert_eq!(2563, result);
    }
}