use std::fs;

fn read_data(path: &str) -> String {
    fs::read_to_string(path).expect("That's not a valid input file")
}

fn part1(data: String) -> i32 {
    data.split("\n\n")
        .map(|elf| elf.split("\n").map(|food| food.parse::<i32>().unwrap()))
        .map(|elf| elf.sum::<i32>())
        .max()
        .unwrap()
}

fn main() {
    let data = read_data("../data/input.txt");

    println!("Part 1: {}", part1(data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_data("data/test.txt");
        assert_eq!(part1(data), 24000)
    }
}
