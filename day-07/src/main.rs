use std::collections::HashMap;
use std::fs;

const MAX_DIR_SIZE: u32 = 100000;
const TOTAL_SPACE: u32 = 70000000;
const REQUIRED_UNUSED_SPACE: u32 = 30000000;

fn solution(path: &str) -> (u32, u32) {
    let input = fs::read_to_string(path).expect("That is not a valid input file");
    let mut stack = vec![String::from("/")];
    let mut sizes = HashMap::<String, u32>::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "$" && words[1] == "cd" {
            // match to cd commands
            match words[2] {
                "/" => {
                    stack.clear();
                    stack.push(String::from("/"))
                }
                ".." => {
                    stack.pop();
                }
                dir => stack.push(dir.to_string()),
            }
        } else if let Ok(filesize) = words[0].parse::<u32>() {
            // matched to files
            for i in 0..stack.len() + 1 {
                let key = stack[0..i].join("/");
                *sizes.entry(key).or_insert(0) += filesize;
            }
        }
    }

    let part_one = sizes.values().filter(|&&size| size < MAX_DIR_SIZE).sum();

    let unused_space = TOTAL_SPACE - sizes["/"];
    let must_delete = REQUIRED_UNUSED_SPACE - unused_space;
    let part_two = sizes
        .values()
        .filter(|&&size| size > must_delete)
        .min()
        .unwrap()
        .clone();
    (part_one, part_two)
}

fn main() {
    let input = "day-07/data/input.txt";
    println!("Part one: {}", solution(input).0);
    println!("Part two: {}", solution(input).1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let test = "data/test.txt";
        assert_eq!(solution(test).0, 95437);
    }

    #[test]
    fn test_part_two() {
        let test = "data/test.txt";
        assert_eq!(solution(test).1, 24933642);
    }
}
