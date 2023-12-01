pub struct Part2;

impl utils::Part for Part2 {
    type Intermediate = Option<u64>;

    type Output = u64;

    fn map(&mut self, line: &str) -> Self::Intermediate {
        let (first_word_opt, last_word_opt) = detect_first_and_last_word(line);
        let first = first_word_opt?.value;
        let last = last_word_opt?.value;
        let total = (first * 10) + last;
    
        Some(total)
    }

    fn reduce(&mut self,input: Vec<Self::Intermediate>) -> Self::Output {
        input.iter()
        .flatten()
        .sum()
    }
}

const WORDS: [(&str, usize, u64); 9] = [
    // word, chars in word, value  -- Maybe with const fn?
    ("one", 3, 1),
    ("two", 3, 2),
    ("three", 5, 3),
    ("four", 4, 4),
    ("five", 4, 5),
    ("six", 3, 6),
    ("seven", 5, 7),
    ("eight", 5, 8),
    ("nine", 4, 9),
];

#[derive(Debug, Clone, Copy)]
pub struct Indexed {
    pub pos: usize,
    pub value: u64,
}
fn starts_with_digit_or_word(input: &str) -> Option<(u64, usize)> {
    let first_char = input.chars().next()?;
    if first_char.is_ascii_digit(){
        let digit = first_char.to_digit(10)?.into();
        return Some((digit, 1)); 
    }
    for word in WORDS {
        if input.starts_with(word.0) {
            return Some((word.2, word.1 - 1)); // -1 to dirty skip overlaps
        }
    }
    None
}

fn detect_first_and_last_word(input: &str) -> (Option<Indexed>, Option<Indexed>) {
    let input_len = input.chars().count();
    let mut pos = 0;
    let mut first_word = None;
    let mut last_word = None;
    while pos < input_len {
        let input = &input[pos..];
        let digit_in_line = starts_with_digit_or_word(input);
        let increment = match digit_in_line {
            Some((value, skip_value)) => {
                let indexed = Indexed { pos, value };
                if first_word.is_none(){
                    // since we iterate through, first will get set only once
                    first_word = Some(indexed);
                }
                last_word = Some(indexed);
                skip_value
            }
            None => 1,
        };
        pos += increment;
    }
    (first_word, last_word)
}

#[cfg(test)]
mod test {
    use utils::Part;

    use crate::part2::Part2;
    #[test]
    fn sample() {
        let input = r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#;

        let mut part = Part2;
        let result: u64 = part.run_part(input);

        println!("{result}");

        assert_eq!(result, 281);
    }

    #[test]
    fn main() {
        let input = include_str!("../inputs/day01.txt");
        let mut part = Part2;
        let result: u64 = part.run_part(input);
        assert_eq!(54985, result);
    }
}
