use itertools::Itertools;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::str::FromStr;

type Coord = (i32, i32);
type Rope = Vec<Coord>;

struct Move {
    direction: Coord,
    steps: usize,
}

const L: Coord = (-1, 0);
const R: Coord = (1, 0);
const U: Coord = (0, 1);
const D: Coord = (0, -1);

fn positions_visited(path: &str, length_of_rope: usize) -> usize {
    let input = read_to_string(path).expect("That is not a valid input");

    let mut rope_coords = vec![vec![(0, 0); length_of_rope]];

    for line in input.lines() {
        let m = Move::from_str(line).unwrap();
        rope_coords = update_coords(rope_coords, m);
    }

    count_tail_positions(rope_coords)
}

fn update_coords(mut ropes: Vec<Rope>, m: Move) -> Vec<Rope> {
    for _ in 0..m.steps {
        let prev = ropes.last().unwrap();
        let new_head = move_knot(prev[0], m.direction);
        let new_rope = new_rope(&prev, new_head);
        ropes.push(new_rope)
    }
    ropes
}

fn new_rope(previous_rope: &Rope, new_head: Coord) -> Rope {
    previous_rope
        .iter()
        .skip(1)
        .fold(vec![new_head], |mut acc, knot| {
            let new_knot = move_tail(&knot, &acc.last().unwrap());
            acc.push(new_knot);
            acc
        })
}

fn move_knot(position: Coord, change: Coord) -> Coord {
    (position.0 + change.0, position.1 + change.1)
}

fn move_tail(tail: &Coord, head: &Coord) -> Coord {
    let (x, y) = (head.0 - tail.0, head.1 - tail.1);
    if x.abs() <= 1 && y.abs() <= 1 {
        *tail
    } else {
        move_knot(*tail, (x.signum(), y.signum()))
    }
}

fn count_tail_positions(ropes: Vec<Rope>) -> usize {
    ropes
        .iter()
        .map(|coord| coord.last().clone())
        .unique()
        .count()
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

fn main() {
    let path = "day-09/data/input.txt";
    println!("Part one: {}", positions_visited(path, 2));
    println!("Part two: {}", positions_visited(path, 10));
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
