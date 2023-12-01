
fn map_line(line: &str) -> Option<u64> {
    
    let first: u64 = line.chars().find(char::is_ascii_digit).and_then(|cha| cha.to_digit(10))?.into();
    let last: u64 = line.chars().rev().find(char::is_ascii_digit).and_then(|cha| cha.to_digit(10))?.into();

    Some((first * 10) + last)
}

pub fn process(input: &str) -> u64{
    input.lines()
        .map(map_line)
        .map(Option::unwrap_or_default)
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    #[test]
    fn sample() {
        let input = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;

        let result: u64 = super::process(input);

        println!("{result}");

        assert_eq!(result, 142);
    }

    #[test]
    fn main() {
        let input = include_str!("../inputs/day01.txt");
    
        assert_eq!(55130, super::process(input));
    }
}