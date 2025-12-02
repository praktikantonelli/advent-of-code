fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-01.txt");
    let output = part1(input);
    println!("{output}");
}

#[derive(Debug)]
struct State {
    current_number: i32,
    range: i32,
}

impl State {
    fn new() -> Self {
        Self {
            current_number: 50,
            range: 100,
        }
    }

    fn turn_left(&mut self, increment: i32) {
        self.current_number = (self.current_number - increment) % self.range;
    }

    fn turn_right(&mut self, increment: i32) {
        self.current_number = (self.current_number + increment) % self.range;
    }
}

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

fn parse(instruction: &str) -> Rotation {
    if instruction.starts_with("R") {
        let increments = instruction.replace("R", "").parse::<i32>().unwrap();
        Rotation::Right(increments)
    } else {
        let increments = instruction.replace("L", "").parse::<i32>().unwrap();
        Rotation::Left(increments)
    }
}

fn part1(input: &str) -> u32 {
    let mut counter = 0;
    let mut state = State::new();
    let instructions = input.lines().map(parse).collect::<Vec<Rotation>>();
    for instruction in instructions {
        match instruction {
            Rotation::Left(increment) => {
                state.turn_left(increment);
            }
            Rotation::Right(increment) => {
                state.turn_right(increment);
            }
        }
        if state.current_number == 0 {
            counter += 1;
        }
    }

    counter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );
        assert_eq!(result, 3);
    }
}
