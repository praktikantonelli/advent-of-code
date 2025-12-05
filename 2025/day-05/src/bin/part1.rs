use regex::Regex;
use std::collections::HashSet;

use itertools::{self, Itertools};

fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-05.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> usize {
    let re = Regex::new(r"\r?\n\r?\n").unwrap();
    let (ranges, ingredients) = re.split(input).collect_tuple().unwrap();
    let ranges = ranges
        .split("\n")
        .map(|x| {
            let (min, max) = x.split("-").collect_tuple().unwrap();
            let min: u64 = min.trim().parse().unwrap();
            let max: u64 = max.trim().parse().unwrap();
            (min..=max).collect()
        })
        .collect::<Vec<Vec<u64>>>()
        .iter()
        .flatten()
        .map(|x| x.to_owned())
        .collect::<Vec<u64>>();

    println!("Ranges collected");
    let fresh_id_set: HashSet<u64> = ranges.into_iter().collect();
    println!("Fresh IDs collected");

    let ingredients = ingredients
        .split("\n")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<HashSet<u64>>();
    println!("Ingredients collected");
    fresh_id_set.intersection(&ingredients).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let output = part1(input);
        assert_eq!(output, 3);
    }
}
