fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-01.txt");
    let output = part2(input);
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
        self.current_number = (self.current_number - increment).rem_euclid(self.range);
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

fn part2(input: &str) -> u32 {
    let mut counter = 0;
    let mut state = State::new();
    let instructions = input.lines().map(parse).collect::<Vec<Rotation>>();
    for instruction in instructions {
        println!("State currently at {}", state.current_number);
        match instruction {
            Rotation::Left(increment) => {
                let old_value = state.current_number;
                println!("Moving {increment} steps to the left!");
                state.turn_left(increment);
                if old_value <= increment {
                    println!("INCREMENT");
                    counter += 1;
                }
            }
            Rotation::Right(increment) => {
                let old_value = state.current_number;
                println!("Moving {increment} steps to the right!");
                state.turn_right(increment);
                if old_value + increment >= state.range {
                    println!("INCREMENT");
                    counter += 1;
                }
            }
        }
    }

    counter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
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
        assert_eq!(result, 6);
    }
}
