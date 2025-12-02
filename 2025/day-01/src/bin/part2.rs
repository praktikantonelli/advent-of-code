fn main() {
    let input = include_str!("../../../../advent-of-code-input/2025/day-01.txt");
    let output = part2(input);
    println!("{output}");
}

#[derive(Debug)]
struct State {
    current_number: i32,
    range: i32,
    counter: i32,
}

impl State {
    fn new() -> Self {
        Self {
            current_number: 50,
            range: 100,
            counter: 0,
        }
    }

    fn turn_left(&mut self, increment: i32) {
        let remainder = increment.rem_euclid(100);
        if self.current_number != 0 && self.current_number <= remainder {
            self.counter += 1;
        }
        self.current_number = (self.current_number - increment).rem_euclid(self.range);
    }

    fn turn_right(&mut self, increment: i32) {
        let remainder = increment.rem_euclid(100);
        if self.current_number + remainder > 99 {
            self.counter += 1;
        }
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

fn part2(input: &str) -> i32 {
    let mut state = State::new();
    let instructions = input.lines().map(parse).collect::<Vec<Rotation>>();
    for instruction in instructions {
        println!("State currently at {}", state.current_number);
        match instruction {
            Rotation::Left(increment) => {
                println!("Moving {increment} steps to the left!");
                state.turn_left(increment);
                state.counter += increment / 100;
            }
            Rotation::Right(increment) => {
                println!("Moving {increment} steps to the right!");
                state.turn_right(increment);
                state.counter += increment / 100;
            }
        }
    }

    state.counter
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
