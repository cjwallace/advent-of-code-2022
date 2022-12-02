use std::fs;

fn part_one(path: &str) -> u32 {
    fs::read_to_string(path)
        .expect("That is not a valid input")
        .split('\n')
        .map(|game| match game {
            "A X" => 1 + 3, // choice + result
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => 0,
        })
        .sum()
}

fn part_two(path: &str) -> u32 {
    fs::read_to_string(path)
        .expect("That is not a valid input")
        .split('\n')
        .map(|game| match game {
            "A X" => 3 + 0, // choice + result
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => 0,
        })
        .sum()
}

fn main() {
    let path = "day-02/data/input.txt";
    println!("Part one: {}", part_one(&path));
    println!("Part two: {}", part_two(&path));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        assert_eq!(part_one(path), 15);
    }

    #[test]
    fn test_part_two() {
        let path = "data/test.txt";
        assert_eq!(part_two(path), 12);
    }
}
