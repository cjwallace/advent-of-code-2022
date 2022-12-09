use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

type Rope = Vec<Coord>;

fn within_one(a: i32, b: i32) -> bool {
    a - 1 <= b && b <= a + 1
}

fn tail_move(tail: &Coord, head: &Coord) -> Coord {
    if within_one(tail.x, head.x) && within_one(tail.y, head.y) {
        return Coord {
            x: tail.x,
            y: tail.y,
        };
    } else if (tail.x == head.x) && (head.y == tail.y + 2) {
        return Coord {
            x: tail.x,
            y: tail.y + 1,
        };
    } else if (tail.x == head.x) && (head.y == tail.y - 2) {
        return Coord {
            x: tail.x,
            y: tail.y - 1,
        };
    } else if (tail.y == head.y) && (head.x == tail.x + 2) {
        return Coord {
            x: tail.x + 1,
            y: tail.y,
        };
    } else if (tail.y == head.y) && (head.x == tail.x - 2) {
        return Coord {
            x: tail.x - 1,
            y: tail.y,
        };
    } else if (head.x > tail.x) && (head.y > tail.y) {
        return Coord {
            x: tail.x + 1,
            y: tail.y + 1,
        };
    } else if (head.x > tail.x) && (head.y < tail.y) {
        return Coord {
            x: tail.x + 1,
            y: tail.y - 1,
        };
    } else if (head.x < tail.x) && (head.y > tail.y) {
        return Coord {
            x: tail.x - 1,
            y: tail.y + 1,
        };
    } else if (head.x < tail.x) && (head.y < tail.y) {
        return Coord {
            x: tail.x - 1,
            y: tail.y - 1,
        };
    } else {
        return Coord {
            x: tail.x,
            y: tail.y,
        };
    }
}

fn new_rope(previous_rope: &Rope, new_head: Coord) -> Rope {
    previous_rope
        .iter()
        .skip(1)
        .fold(vec![new_head], |mut acc, knot| {
            let new_knot = tail_move(&knot, &acc.last().unwrap());
            acc.push(new_knot);
            acc
        })
}

fn solution(path: &str, length_of_rope: usize) -> i32 {
    let input = read_to_string(path).expect("That is not a valid input");

    let mut coords = vec![vec![Coord { x: 0, y: 0 }; length_of_rope]];

    for line in input.lines() {
        let (direction, steps_str) = line.split_once(' ').unwrap();
        let steps = steps_str.parse::<i32>().unwrap();

        match direction {
            "L" => {
                for _ in 0..steps {
                    let prev = coords.last().unwrap();
                    let new_head = Coord {
                        x: prev[0].x - 1,
                        y: prev[0].y,
                    };
                    let new_rope = new_rope(&prev, new_head);
                    coords.push(new_rope);
                }
            }
            "R" => {
                for _ in 0..steps {
                    let prev = coords.last().unwrap();
                    let new_head = Coord {
                        x: prev[0].x + 1,
                        y: prev[0].y,
                    };
                    let new_rope = new_rope(&prev, new_head);
                    coords.push(new_rope);
                }
            }
            "U" => {
                for _ in 0..steps {
                    let prev = coords.last().unwrap();
                    let new_head = Coord {
                        x: prev[0].x,
                        y: prev[0].y + 1,
                    };
                    let new_rope = new_rope(&prev, new_head);
                    coords.push(new_rope);
                }
            }
            "D" => {
                for _ in 0..steps {
                    let prev = coords.last().unwrap();
                    let new_head = Coord {
                        x: prev[0].x,
                        y: prev[0].y - 1,
                    };
                    let new_rope = new_rope(&prev, new_head);
                    coords.push(new_rope);
                }
            }
            _ => unreachable!("Oh no."),
        }
    }

    coords
        .iter()
        .map(|coord| coord.last().clone())
        .unique()
        .count() as i32
}

fn main() {
    let path = "day-09/data/input.txt";
    println!("Part one: {}", solution(path, 2));
    println!("Part two: {}", solution(path, 10));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        assert_eq!(solution(path, 2), 13);
    }

    #[test]
    fn test_part_two() {
        let path_one = "data/test.txt";
        assert_eq!(solution(path_one, 10), 1);
        let path_two = "data/test_part_two.txt";
        assert_eq!(solution(path_two, 10), 36)
    }
}
