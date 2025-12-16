fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-06.txt");
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
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let output = part1(input);
        assert_eq!(output, 4277556);
    }
}
