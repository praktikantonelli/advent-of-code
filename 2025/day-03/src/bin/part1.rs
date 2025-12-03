fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-03.txt").trim();
    let output = part1(input);
    println!("{output}");
}

fn find_max_value_and_index(chars: &[u32]) -> (usize, u32) {
    let mut current_max: u32 = 0;
    let mut current_index = 0;
    for (index, value) in chars.iter().enumerate() {
        if *value > current_max {
            current_max = *value;
            current_index = index;
        }
    }
    (current_index, current_max)
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for batteries in input.lines() {
        let length = batteries.len();
        let batteries_as_chars: Vec<u32> =
            batteries.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let (first_index, first_value) =
            find_max_value_and_index(&batteries_as_chars[0..length - 1]);
        let (_, second_value) =
            find_max_value_and_index(&batteries_as_chars[first_index + 1..length]);
        let max_joltage = format!("{first_value}{second_value}")
            .parse::<u32>()
            .unwrap();
        sum += max_joltage;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111
";
        let output = part1(input);
        assert_eq!(output, 357);
    }
}
