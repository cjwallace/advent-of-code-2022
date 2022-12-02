use itertools::Itertools;
use std::fs;

fn calories(path: &str, n: usize) -> u32 {
    fs::read_to_string(path)
        .expect("That's not a valid input file")
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted()
        .rev()
        .take(n)
        .sum()
}

fn main() {
    let input = "day-01/data/input.txt";

    println!("Part one: {}", calories(input, 1));
    println!("Part two: {}", calories(input, 3));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let test = "data/test.txt";
        assert_eq!(calories(test, 1), 24000);
    }

    #[test]
    fn test_part_two() {
        let test = "data/test.txt";
        assert_eq!(calories(test, 3), 45000)
    }
}
