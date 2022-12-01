use itertools::Itertools;
use std::fs;

fn read_data(path: &str) -> String {
    fs::read_to_string(path).expect("That's not a valid input file")
}

fn data_to_calories(data: String) -> Vec<u32> {
    data.split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum()
        })
        .collect()
}

fn part1(calories: &[u32]) -> u32 {
    *calories.iter().max().unwrap()
}

fn part2(calories: &[u32]) -> u32 {
    calories.iter().sorted().rev().take(3).sum()
}

fn main() {
    let data = read_data("day-01/data/input.txt");
    let calories = data_to_calories(data);

    println!("Part 1: {}", part1(&calories));
    println!("Part 2: {}", part2(&calories));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_data("data/test.txt");
        let calories = data_to_calories(data);
        assert_eq!(part1(&calories), 24000)
    }

    #[test]
    fn test_part2() {
        let data = read_data("data/test.txt");
        let calories = data_to_calories(data);
        assert_eq!(part2(&calories), 45000)
    }
}
