fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-04.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let output = part1(input);
        assert_eq!(output, 13);
    }
}
