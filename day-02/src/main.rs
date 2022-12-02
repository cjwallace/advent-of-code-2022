use std::fs;

fn solution(path: &str, match_fn: fn(&str) -> u32) -> u32 {
    fs::read_to_string(path)
        .expect("That is not a valid input")
        .split('\n')
        .map(match_fn)
        .sum()
}

fn match_one(game: &str) -> u32 {
    match game {
        // pattern => choice + result
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => 0,
    }
}

fn match_two(game: &str) -> u32 {
    match game {
        // pattern => choice + result
        "A X" => 3 + 0,
        "A Y" => 1 + 3,
        "A Z" => 2 + 6,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 2 + 0,
        "C Y" => 3 + 3,
        "C Z" => 1 + 6,
        _ => 0,
    }
}

fn main() {
    let path = "day-02/data/input.txt";
    println!("Part one: {}", solution(path, match_one));
    println!("Part two: {}", solution(path, match_two));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        assert_eq!(solution(path, match_one), 15);
    }

    #[test]
    fn test_part_two() {
        let path = "data/test.txt";
        assert_eq!(solution(path, match_two), 12);
    }
}
