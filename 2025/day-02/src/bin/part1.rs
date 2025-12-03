use itertools::Itertools;
use pcre2::bytes::Regex;

fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-02.txt").trim();
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> u64 {
    let mut sum: u64 = 0;
    let re = Regex::new(r"^([0-9]+)\1$").unwrap();
    for range in input.split(",") {
        dbg!(range);
        let (min, max) = range.split("-").collect_tuple().unwrap();
        let min = min.parse::<u64>().unwrap();
        let max = max.parse::<u64>().unwrap();
        for number in min..=max {
            let number_str = number.to_string();
            if re.is_match(number_str.as_bytes()).unwrap() {
                println!("{number}");
                sum += number;
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(result, 1227775554);
    }
}
