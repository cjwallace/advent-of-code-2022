use std::str::FromStr;
use std::{error, fs};

fn main() {
    let path = "day-10/data/input.txt";
    println!("Part one: {}", part_one(path));
    println!("Part two:");
    part_two(path);
}

fn part_one(path: &str) -> i32 {
    let input = fs::read_to_string(path).expect("That is not a valid input");
    let cycles = clock_circuit(&input);

    let signal_strength = 20 * cycles[19]
        + 60 * cycles[59]
        + 100 * cycles[99]
        + 140 * cycles[139]
        + 180 * cycles[179]
        + 220 * cycles[219];
    signal_strength
}

fn part_two(path: &str) {
    let input = fs::read_to_string(path).expect("That is not a valid input");
    let cycles = clock_circuit(&input);

    let screen: Vec<&str> = cycles
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if (i.rem_euclid(40) as i32 - x).abs() <= 1 {
                "#"
            } else {
                "."
            }
        })
        .collect();

    print_crt(screen);
}

fn clock_circuit(input: &str) -> Vec<i32> {
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

    cycles
}

fn print_crt(v: Vec<&str>) {
    let line_length = 40;
    for i in 0..6 {
        print_crt_line(&v[i * line_length..(i + 1) * line_length]);
    }
}

fn print_crt_line(output: &[&str]) {
    for s in output {
        print!("{}", s);
    }
    println!();
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

    #[test]
    fn test_part_two() {
        let path = "data/test.txt";
        part_two(path);
    }
}
