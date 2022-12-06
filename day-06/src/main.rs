use std::{collections::HashSet, fs};

fn start_of_packet(packet: &String, marker_length: usize) -> usize {
    let chars: Vec<char> = packet.chars().collect();
    for (i, window) in chars.windows(marker_length).enumerate() {
        if HashSet::<char>::from_iter(window.to_owned()).len() == marker_length {
            return marker_length + i;
        }
    }
    return 0;
}

fn main() {
    let path = "day-06/data/input.txt";
    let packet = fs::read_to_string(path).expect("That's not a valid input file");

    println!("Part one: {}", start_of_packet(&packet, 4));
    println!("Part two: {}", start_of_packet(&packet, 14));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            start_of_packet(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4),
            5
        );
        assert_eq!(
            start_of_packet(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4),
            6
        );
        assert_eq!(
            start_of_packet(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4),
            10
        );
        assert_eq!(
            start_of_packet(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4),
            11
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            start_of_packet(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 14),
            19
        );
        assert_eq!(
            start_of_packet(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 14),
            23
        );
        assert_eq!(
            start_of_packet(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(), 14),
            23
        );
        assert_eq!(
            start_of_packet(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 29),
            29
        );
        assert_eq!(
            start_of_packet(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 26),
            26
        );
    }
}
