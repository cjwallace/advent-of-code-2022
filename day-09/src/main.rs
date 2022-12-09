use itertools::Itertools;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::str::FromStr;

const L: Coord = (-1, 0);
const R: Coord = (1, 0);
const U: Coord = (0, 1);
const D: Coord = (0, -1);

fn main() {
    let path = "day-09/data/input.txt";
    println!("Part one: {}", positions_visited(path, 2));
    println!("Part two: {}", positions_visited(path, 10));
}

fn positions_visited(path: &str, length_of_rope: usize) -> usize {
    let input = read_to_string(path).expect("That is not a valid input");

    let mut rope = RopePositions {
        current: vec![(0, 0); length_of_rope],
        history: vec![],
    };

    for line in input.lines() {
        let m = Move::from_str(line).unwrap();
        rope = rope.update(m);
    }

    rope.count_tail_positions()
}

type Coord = (i32, i32);
type Rope = Vec<Coord>;

struct RopePositions {
    current: Rope,
    history: Vec<Rope>,
}

impl RopePositions {
    fn update(mut self, m: Move) -> Self {
        for _ in 0..m.steps {
            let new_head = self.move_knot(&self.current[0], &m.direction);
            let new_rope = self.new_rope(&self.current, new_head);
            self.history.push(self.current);
            self.current = new_rope;
        }
        self
    }

    fn count_tail_positions(&self) -> usize {
        vec![self.current.clone()]
            .iter()
            .chain(self.history.iter())
            .map(|coord| coord.last().unwrap())
            .unique()
            .count()
    }

    fn new_rope(&self, previous_rope: &Rope, new_head: Coord) -> Rope {
        previous_rope
            .iter()
            .skip(1)
            .fold(vec![new_head], |mut acc, knot| {
                let new_knot = self.move_tail(&knot, &acc.last().unwrap());
                acc.push(new_knot);
                acc
            })
    }

    fn move_tail(&self, tail: &Coord, head: &Coord) -> Coord {
        let (x, y) = (head.0 - tail.0, head.1 - tail.1);
        if x.abs() <= 1 && y.abs() <= 1 {
            *tail
        } else {
            self.move_knot(tail, &(x.signum(), y.signum()))
        }
    }

    fn move_knot(&self, position: &Coord, change: &Coord) -> Coord {
        (position.0 + change.0, position.1 + change.1).clone()
    }
}

struct Move {
    direction: Coord,
    steps: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction_str, steps_str) = s.split_once(' ').unwrap();

        let direction = match direction_str {
            "L" => L,
            "R" => R,
            "U" => U,
            "D" => D,
            _ => unreachable!("Oh no."),
        };

        let steps = usize::from_str_radix(steps_str, 10)?;

        Ok(Move { direction, steps })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        assert_eq!(positions_visited(path, 2), 13);
    }

    #[test]
    fn test_part_two() {
        let path_one = "data/test.txt";
        assert_eq!(positions_visited(path_one, 10), 1);
        let path_two = "data/test_part_two.txt";
        assert_eq!(positions_visited(path_two, 10), 36)
    }
}
