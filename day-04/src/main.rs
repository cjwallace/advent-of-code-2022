use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

type Section = u32;

#[derive(Clone, Copy)]
struct Elf {
    lower: Section,
    upper: Section,
}

impl FromStr for Elf {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lower, upper) = s.split_once('-').unwrap();
        Ok(Elf {
            lower: lower.parse::<u32>()?,
            upper: upper.parse::<u32>()?,
        })
    }
}

fn solution(path: &str, condition: fn(Elf, Elf) -> bool) -> usize {
    fs::read_to_string(path)
        .expect("That is not a valid input file")
        .split('\n')
        .filter(|line| {
            let (first_elf, second_elf) = line.split_once(',').unwrap();
            let snowball = Elf::from_str(first_elf).unwrap();
            let peppermint = Elf::from_str(second_elf).unwrap();
            condition(snowball, peppermint) | condition(peppermint, snowball)
        })
        .count()
}

fn contains(snowball: Elf, peppermint: Elf) -> bool {
    (snowball.lower <= peppermint.lower) & (snowball.upper >= peppermint.upper)
}

fn overlaps(snowball: Elf, peppermint: Elf) -> bool {
    (snowball.upper >= peppermint.lower) & (peppermint.upper >= snowball.lower)
}
fn main() {
    let path = "day-04/data/input.txt";
    println!("Part one: {}", solution(path, contains));
    println!("Part two: {}", solution(path, overlaps));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        assert_eq!(solution(path, contains), 2);
    }

    #[test]
    fn test_part_two() {
        let path = "data/test.txt";
        assert_eq!(solution(path, overlaps), 4);
    }
}
