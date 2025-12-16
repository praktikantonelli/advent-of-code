use std::cmp::max;

use regex::Regex;

use itertools::{self, Itertools};

fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-05.txt");
    let output = part2(input);
    println!("{output}");
}

fn part2(input: &str) -> u64 {
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
            (min, max)
        })
        .collect::<Vec<(u64, u64)>>();

    // Sort by min => overlapping ranges are always next to each other
    ranges.sort_unstable();

    // Iterate over (min, max) tuples, figure out if current and next range are
    // overlapping/adjacent or completely disjoint
    let mut num_unique_values: u64 = 0;
    let mut current_min = ranges[0].0;
    let mut current_max = ranges[0].1;
    for &(min_val, max_val) in &ranges[1..] {
        if min_val <= current_max + 1 {
            // Overlapping or adjacent
            current_max = max(current_max, max_val);
        } else {
            // Disjoint => Count values in range and move current min/max to iterator
            num_unique_values += current_max - current_min + 1;
            current_min = min_val;
            current_max = max_val;
        }
    }

    // End of loop: Handle last range
    num_unique_values += current_max - current_min + 1;
    num_unique_values
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
