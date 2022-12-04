use std::fs;

fn condition_one(fl: u32, fu: u32, sl: u32, su: u32) -> bool {
    ((fl <= sl) & (fu >= su)) | ((fl >= sl) & (fu <= su))
}

fn condition_two(fl: u32, fu: u32, sl: u32, su: u32) -> bool {
    ((fu >= sl) & (su >= fl)) | ((su >= fl) & (fu >= sl))
}

fn parse_sections(elf: &str) -> (u32, u32) {
    let (lower, upper) = elf.split_once('-').unwrap();
    (lower.parse::<u32>().unwrap(), upper.parse::<u32>().unwrap())
}

fn solution(path: &str, condition: fn(u32, u32, u32, u32) -> bool) -> usize {
    fs::read_to_string(path)
        .expect("That is not a valid input file")
        .split('\n')
        .filter(|line| {
            let (first_elf, second_elf) = line.split_once(",").unwrap();
            let (first_lower, first_upper) = parse_sections(first_elf);
            let (second_lower, second_upper) = parse_sections(second_elf);
            condition(first_lower, first_upper, second_lower, second_upper)
        })
        .count()
}

fn main() {
    let path = "day-04/data/input.txt";
    println!("Part one: {}", solution(path, condition_one));
    println!("Part two: {}", solution(path, condition_two));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        assert_eq!(solution(path, condition_one), 2);
    }

    #[test]
    fn test_part_two() {
        let path = "data/test.txt";
        assert_eq!(solution(path, condition_two), 4);
    }
}
