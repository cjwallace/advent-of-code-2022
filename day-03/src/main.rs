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

fn main() {
    let path = "day-03/data/input.txt";
    println!("Part one: {}", part_one(path));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        assert_eq!(part_one(path), 157);
    }
}
