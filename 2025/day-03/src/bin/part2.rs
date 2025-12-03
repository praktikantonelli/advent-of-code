fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-03.txt").trim();
    let output = part2(input);
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

fn part2(input: &str) -> u64 {
    let mut sum = 0;
    for batteries in input.lines() {
        let length = batteries.len();
        let num_required_battieres = 12;
        let batteries_as_chars: Vec<u32> =
            batteries.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut next_index = 0;
        let mut found_batteries = vec![];

        for remaining_batteries in (0..num_required_battieres).rev() {
            println!("next index: {next_index}");
            println!("max index: {}", length - remaining_batteries);
            let (index, digit) = find_max_value_and_index(
                &batteries_as_chars[next_index..length - remaining_batteries],
            );
            println!("index {index} has max value: {digit}");
            next_index = next_index + index + 1;
            found_batteries.push(digit);
        }
        let max_joltage = found_batteries
            .iter()
            .map(|digit| digit.to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse::<u64>()
            .unwrap();
        dbg!(max_joltage);
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
        let output = part2(input);
        assert_eq!(output, 3121910778619);
    }
}
