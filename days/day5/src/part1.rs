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

    #[test]
    fn sample_input(){
        use utils::Part;
        let input = r#"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"#;

        let mut part = super::Part1;

        let res = part.run_part(input);

        assert_eq!(res, 9999); //todo
    }

    #[test]
    fn main() {
        use utils::Part;
        let input = include_str!("../inputs/day5.txt");

        let mut part = super::Part1;

        let result = part.run_part(input);
        assert_eq!(9999, result); //todo
    }
}