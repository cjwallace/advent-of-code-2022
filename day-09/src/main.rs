use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    head: Coord,
    tail: Coord,
}

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

fn part_one(path: &str) -> i32 {
    let input = read_to_string(path).expect("That is not a valid input");

    let mut coords = vec![Rope {
        head: Coord { x: 0, y: 0 },
        tail: Coord { x: 0, y: 0 },
    }];

    for line in input.lines() {
        let (direction, steps_str) = line.split_once(' ').unwrap();
        let steps = steps_str.parse::<i32>().unwrap();

        match direction {
            "L" => {
                for _ in 0..steps {
                    let prev = coords.last().unwrap();
                    let new_head = Coord {
                        x: prev.head.x - 1,
                        y: prev.head.y,
                    };
                    let new_tail = tail_move(&prev.tail, &new_head);
                    coords.push(Rope {
                        head: new_head,
                        tail: new_tail,
                    });
                }
            }
            "R" => {
                for _ in 0..steps {
                    let prev = coords.last().unwrap();
                    let new_head = Coord {
                        x: prev.head.x + 1,
                        y: prev.head.y,
                    };
                    let new_tail = tail_move(&prev.tail, &new_head);
                    coords.push(Rope {
                        head: new_head,
                        tail: new_tail,
                    });
                }
            }
            "U" => {
                for _ in 0..steps {
                    let prev = coords.last().unwrap();
                    let new_head = Coord {
                        x: prev.head.x,
                        y: prev.head.y + 1,
                    };
                    let new_tail = tail_move(&prev.tail, &new_head);
                    coords.push(Rope {
                        head: new_head,
                        tail: new_tail,
                    });
                }
            }
            "D" => {
                for _ in 0..steps {
                    let prev = coords.last().unwrap();
                    let new_head = Coord {
                        x: prev.head.x,
                        y: prev.head.y - 1,
                    };
                    let new_tail = tail_move(&prev.tail, &new_head);
                    coords.push(Rope {
                        head: new_head,
                        tail: new_tail,
                    });
                }
            }
            _ => unreachable!("Oh no."),
        }
    }
    println!("{:?}", coords);

    coords
        .iter()
        .map(|coord| coord.tail.clone())
        .unique()
        .count() as i32
}

fn main() {
    let path = "day-09/data/input.txt";
    println!("Part one: {}", part_one(path));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        assert_eq!(part_one(path), 13)
    }
}
