use regex::Regex;

use itertools::{self, Itertools};

fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-05.txt");
    let output = part2(input);
    println!("{output}");
}

fn part2(input: &str) -> usize {
    let mut counter = 0;
    let re = Regex::new(r"\r?\n\r?\n").unwrap();
    let (ranges, _ingredients) = re.split(input).collect_tuple().unwrap();
    let mut ranges = ranges
        .split("\n")
        .map(|x| {
            counter += 1;
            println!("{counter}");
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

    ranges.sort_unstable();
    ranges.dedup();
    ranges.len()
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
        let output = part2(input);
        assert_eq!(output, 14);
    }
}
