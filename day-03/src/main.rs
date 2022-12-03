use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn part_one(path: &str) -> u32 {
    fs::read_to_string(path)
        .expect("That's not a valid input file")
        .split('\n')
        .map(|pack| {
            let mid = (pack.len() / 2) as usize;
            let (first, last) = pack.split_at(mid);
            let a = HashSet::<char>::from_iter(first.chars());
            let b = HashSet::<char>::from_iter(last.chars());
            let c = a.intersection(&b).next().unwrap().to_owned();
            if c.is_lowercase() {
                c as u32 - 'a' as u32 + 1
            } else {
                c as u32 - 'A' as u32 + 27
            }
        })
        .sum()
}

fn part_two(path: &str) -> u32 {
    fs::read_to_string(path)
        .expect("That's not a valid input file")
        .split('\n')
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let elves = chunk.map(|elf| HashSet::<char>::from_iter(elf.chars()));
            let badge = elves
                .reduce(|common_items, elf| common_items.intersection(&elf).cloned().collect())
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .to_owned();

            if badge.is_lowercase() {
                badge as u32 - 'a' as u32 + 1
            } else {
                badge as u32 - 'A' as u32 + 27
            }
        })
        .sum()
}

fn main() {
    let path = "day-03/data/input.txt";
    println!("Part one: {}", part_one(path));
    println!("Part one: {}", part_two(path));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        assert_eq!(part_one(path), 157);
    }

    #[test]
    fn test_part_two() {
        let path = "data/test.txt";
        assert_eq!(part_two(path), 70);
    }
}
