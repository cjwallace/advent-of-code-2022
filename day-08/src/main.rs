use std::fs;

type Forest = Vec<Vec<u32>>;
type Coord = (usize, usize);

fn check_visibility(trees: &Forest, (r, c): Coord) -> bool {
    if r == 0 || c == 0 || r == trees.len() - 1 || c == trees[0].len() - 1 {
        return true;
    }

    let row = trees[r].clone();
    let col: Vec<u32> = trees
        .iter()
        .map(|row| row.iter().nth(c).unwrap().clone())
        .collect();
    let tree = trees[r][c];

    let visible_left = row[0..c].iter().max().unwrap() < &tree;
    let visible_right = row[c + 1..].iter().max().unwrap() < &tree;
    let visible_top = col[0..r].iter().max().unwrap() < &tree;
    let visible_bottom = col[r + 1..].iter().max().unwrap() < &tree;
    visible_left || visible_right || visible_top || visible_bottom
}

fn part_one(path: &str) -> u32 {
    let trees: Forest = fs::read_to_string(path)
        .expect("That is not a valid input")
        .lines()
        .map(|row| row.chars().map(|tree| tree.to_digit(10).unwrap()).collect())
        .collect();

    let mut visibility = vec![vec![false; trees[0].len()]; trees.len()];

    for (r, row) in trees.iter().enumerate() {
        for (c, _col) in row.iter().enumerate() {
            visibility[r][c] = check_visibility(&trees, (r, c));
        }
    }

    visibility
        .iter()
        .map(|row| {
            row.iter()
                .map(|&visible| if visible { 1 } else { 0 })
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    let path = "day-08/data/input.txt";
    println!("Part one: {}", part_one(path));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        assert_eq!(part_one(path), 21);
    }
}
