use std::collections::HashSet;
use std::fs;

type Pack = (HashSet<char>, HashSet<char>);

fn packs(path: &str) -> Vec<Pack> {
    fs::read_to_string(path)
        .expect("That's not a valid input file")
        .split('\n')
        .map(|pack| {
            let compartments = pack.split_at(pack.len() / 2);
            (
                compartments.0.chars().collect(),
                compartments.1.chars().collect(),
            )
        })
        .collect::<Vec<Pack>>()
}

fn points(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn part_one(packs: &[Pack]) -> u32 {
    packs
        .iter()
        .map(|pack| {
            let (one, two) = pack;
            let item = one.intersection(two).next().unwrap().to_owned();
            points(item)
        })
        .sum()
}

fn part_two(packs: &[Pack]) -> u32 {
    packs
        .chunks(3)
        .map(|elves| {
            let badge = elves
                .iter()
                // combine compartments
                .map(|pack| pack.0.union(&pack.1).cloned().collect())
                // find common item
                .reduce(|common_items: HashSet<char>, pack: HashSet<char>| {
                    common_items.intersection(&pack).cloned().collect()
                })
                // rust is silly
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .to_owned();

            points(badge)
        })
        .sum()
}

fn main() {
    let path = "day-03/data/input.txt";
    let packs = packs(path);
    println!("Part one: {}", part_one(&packs));
    println!("Part one: {}", part_two(&packs));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        let packs = packs(path);
        assert_eq!(part_one(&packs), 157);
    }

    #[test]
    fn test_part_two() {
        let path = "data/test.txt";
        let packs = packs(path);
        assert_eq!(part_two(&packs), 70);
    }
}
