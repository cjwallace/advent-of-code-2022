use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::fs;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Crates {
    Crate(char),
    Space,
}

type Stack = Vec<Crates>;

#[derive(Debug)]
struct Instruction {
    source: usize,
    target: usize,
    number: usize,
}

#[derive(PartialEq)]
enum Crane {
    CrateMover9000,
    CrateMover9001,
}

/// A crate with a character inside it
fn parse_crate(input: &str) -> IResult<&str, Crates> {
    let parser = delimited(tag("["), alpha1, tag("]"));
    map(parser, |c: &str| Crates::Crate(c.parse::<char>().unwrap()))(input)
}

/// The empty space where a crate could be
fn parse_space(input: &str) -> IResult<&str, Crates> {
    let parser = tag("   ");
    map(parser, |_: &str| Crates::Space)(input)
}

/// A horizontal row of crates and spaces
fn parse_row_of_crates(input: &str) -> IResult<&str, Stack> {
    let crate_or_space = alt((parse_crate, parse_space));
    separated_list0(tag(" "), crate_or_space)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let number = preceded(tag("move "), digit1);
    let from = preceded(tag(" from "), digit1);
    let to = preceded(tag(" to "), digit1);
    let parser = tuple((number, from, to));
    map(parser, |(n, f, t): (&str, &str, &str)| Instruction {
        source: f.parse::<usize>().unwrap(),
        target: t.parse::<usize>().unwrap(),
        number: n.parse::<usize>().unwrap(),
    })(input)
}

fn apply_instructions(
    instructions: Vec<Instruction>,
    mut crates: Vec<Stack>,
    crane: Crane,
) -> Vec<Stack> {
    instructions.iter().for_each(|inst| {
        let index_of_split = crates[inst.source - 1].len() - inst.number;
        let mut crates_to_move: Stack = crates[inst.source - 1].split_off(index_of_split).into();
        if crane == Crane::CrateMover9000 {
            crates_to_move = crates_to_move.into_iter().rev().collect()
        }
        crates[inst.target - 1].extend(crates_to_move)
    });
    crates
}

fn solution(path: &str, crane: Crane) -> String {
    // lots of parsing
    let input = fs::read_to_string(path).expect("That is not a valid input file");
    let (crates_str, instruction_str) = input.split_once("\n\n").unwrap();

    let (crates_without_indicies, _) = crates_str.split_once("\n 1").unwrap();
    let crate_rows: Vec<Stack> = crates_without_indicies
        .split('\n')
        .map(|row| parse_row_of_crates(row).unwrap().1)
        .collect();

    let mut crate_stacks: Vec<Stack> = (0..crate_rows[0].len())
        .map(|ix| crate_rows.iter().map(|row| row[ix]).collect::<Stack>())
        .map(|stack| {
            stack
                .into_iter()
                .filter(|cr| !matches!(cr, Crates::Space))
                .rev()
                .collect()
        })
        .collect();

    let instructions: Vec<Instruction> = instruction_str
        .split('\n')
        .map(|instruction| parse_instruction(instruction).unwrap().1)
        .collect();

    // finally, apply instructions to crate stacks
    crate_stacks = apply_instructions(instructions, crate_stacks, crane);

    crate_stacks
        .iter()
        .map(|stack| *stack.last().unwrap())
        .map(|cr| {
            if let Crates::Crate(character) = cr {
                character
            } else {
                ' '
            }
        })
        .collect::<String>()
}

fn main() {
    let path = "day-05/data/input.txt";
    println!("Part one: {:?}", solution(path, Crane::CrateMover9000));
    println!("Part two: {:?}", solution(path, Crane::CrateMover9001));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        assert_eq!(solution(path, Crane::CrateMover9000), "CMZ");
    }

    #[test]
    fn test_part_two() {
        let path = "data/test.txt";
        assert_eq!(solution(path, Crane::CrateMover9001), "MCD");
    }

    #[test]
    fn test_parse_crate() {
        let (_, output) = parse_crate("[A]").unwrap();
        assert_eq!(output, Crates::Crate('A'));
    }

    #[test]
    fn test_parse_space() {
        let (_, output) = parse_space("   ").unwrap();
        assert_eq!(output, Crates::Space);

        let (remainder, output) = parse_space("    ").unwrap();
        assert_eq!(output, Crates::Space);
        assert_eq!(remainder, " ");
    }

    #[test]
    fn test_parse_row_of_crates() {
        let (_, output) = parse_row_of_crates("    [A] [B]").unwrap();
        assert_eq!(
            output,
            vec![Crates::Space, Crates::Crate('A'), Crates::Crate('B')]
        );

        let (_, output) = parse_row_of_crates("           ").unwrap();
        assert_eq!(output, vec![Crates::Space, Crates::Space, Crates::Space]);
    }
}
