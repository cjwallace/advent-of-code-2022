use std::fs;

fn part_one(path: &str) -> u32 {
    fs::read_to_string(path)
        .expect("That is not a valid input file")
        .split('\n')
        .filter(|line| {
            let (first_elf, second_elf) = line.split_once(",").unwrap();
            let (first_lower, first_upper) = first_elf.split_once('-').unwrap();
            let (second_lower, second_upper) = second_elf.split_once('-').unwrap();
            let fl = first_lower.parse::<u32>().unwrap();
            let fu = first_upper.parse::<u32>().unwrap();
            let sl = second_lower.parse::<u32>().unwrap();
            let su = second_upper.parse::<u32>().unwrap();
            ((fl <= sl) & (fu >= su)) | ((fl >= sl) & (fu <= su))
        })
        .count() as u32
}

fn main() {
    let path = "day-04/data/input.txt";
    println!("Part one: {}", part_one(path))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        assert_eq!(part_one(path), 2);
    }
}
