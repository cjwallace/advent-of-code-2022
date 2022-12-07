use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::fs;

type Directory = String;

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
enum Command {
    CD(Directory),
    LS,
}

#[derive(Debug)]
enum FS {
    Directory(Directory),
    File(File),
    Command(Command),
}

fn directory_parser(input: &str) -> IResult<&str, FS> {
    let parser = preceded(tag("dir "), alpha1);
    map(parser, |name: &str| FS::Directory(name.to_string()))(input)
}

fn ls_parser(input: &str) -> IResult<&str, FS> {
    let parser = tag("$ ls");
    map(parser, |_: &str| FS::Command(Command::LS))(input)
}

fn cd_parser(input: &str) -> IResult<&str, FS> {
    let dir_parser = alt((alpha1, tag("/"), tag("..")));
    let parser = preceded(tag("$ cd "), dir_parser);
    map(parser, |dir: &str| {
        FS::Command(Command::CD(dir.to_string()))
    })(input)
}

fn command_parser(input: &str) -> IResult<&str, FS> {
    alt((cd_parser, ls_parser))(input)
}

fn file_parser(input: &str) -> IResult<&str, FS> {
    let parser = separated_pair(digit1, tag(" "), alpha1);
    map(parser, |(size, name): (&str, &str)| {
        FS::File(File {
            size: size.parse::<usize>().unwrap(),
            name: name.to_string(),
        })
    })(input)
}

fn parser(input: &str) -> IResult<&str, FS> {
    alt((command_parser, file_parser, directory_parser))(input)
}

fn parse(path: &str) -> Vec<FS> {
    fs::read_to_string(path)
        .expect("That is not a valid input file")
        .split('\n')
        .map(|line| parser(line).unwrap().1)
        .collect::<Vec<FS>>()
}

fn part_one(path: &str) -> u32 {
    let commands = parse(path);
    println!("{:?}", commands);
    0
}

fn main() {
    let input = "day-07/data/input.txt";

    println!("Part one: {}", part_one(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let test = "data/test.txt";
        assert_eq!(part_one(test), 95437);
    }
}
