use std::fs;

fn main() {
    let path = "day-10/data/input.txt";
    let input = fs::read_to_string(path).expect("That is not a valid input");
    let cycles = clock_circuit(&input);
    println!("Part one: {}", part_one(&cycles));
    println!("Part two:");
    part_two(&cycles);
}

fn part_one(cycles: &Vec<i32>) -> i32 {
    20 * cycles[19]
        + 60 * cycles[59]
        + 100 * cycles[99]
        + 140 * cycles[139]
        + 180 * cycles[179]
        + 220 * cycles[219]
}

fn part_two(cycles: &Vec<i32>) {
    let screen: Vec<char> = cycles
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if (i.rem_euclid(40) as i32 - x).abs() <= 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect();

    let pixels: String = screen
        .chunks(40)
        .map(|row| String::from_iter(row.iter()))
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", pixels);
}

fn clock_circuit(input: &str) -> Vec<i32> {
    let mut register = 1;
    let mut cycles = Vec::<i32>::new();

    for line in input.lines() {
        if line.starts_with("addx") {
            let (_, n) = line.split_once(' ').unwrap();
            cycles.push(register);
            cycles.push(register);
            register += n.parse::<i32>().unwrap();
        } else {
            cycles.push(register);
        }
    }

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        let input = fs::read_to_string(path).expect("That is not a valid input");
        let cycles = clock_circuit(&input);
        assert_eq!(part_one(&cycles), 13140);
    }

    #[test]
    fn test_part_two() {
        let path = "data/test.txt";
        let input = fs::read_to_string(path).expect("That is not a valid input");
        let cycles = clock_circuit(&input);
        part_two(&cycles);
    }
}
