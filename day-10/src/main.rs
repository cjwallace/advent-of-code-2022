use std::str::FromStr;
use std::{error, fs};

fn main() {
    let path = "day-10/data/input.txt";
    println!("Part one: {}", part_one(path));
}

fn part_one(path: &str) -> i32 {
    let input = fs::read_to_string(path).expect("That is not a valid input");
    let mut register = 1;
    let mut cycles = Vec::<i32>::new();

    for line in input.lines() {
        let instruction = Instruction::from_str(line).unwrap();
        match instruction {
            Instruction::NoOp => {
                cycles.push(register);
            }
            Instruction::AddX(x) => {
                cycles.push(register);
                cycles.push(register);
                register += x;
            }
        }
    }

    let signal_strength = 20 * cycles[19]
        + 60 * cycles[59]
        + 100 * cycles[99]
        + 140 * cycles[139]
        + 180 * cycles[179]
        + 220 * cycles[219];
    signal_strength
}

enum Instruction {
    AddX(i32),
    NoOp,
}

impl FromStr for Instruction {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("addx") {
            let (_, n) = s.split_once(' ').unwrap();
            Ok(Instruction::AddX(n.parse::<i32>().unwrap()))
        } else {
            Ok(Instruction::NoOp)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        assert_eq!(part_one(path), 13140);
    }
}
