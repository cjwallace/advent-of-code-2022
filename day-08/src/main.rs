use std::fs;

type Map<T> = Vec<Vec<T>>;
type Tree = u32;
type Grove = Map<Tree>;
type Coord = (usize, usize);

fn part_one(trees: Grove) -> u32 {
    let visibility = quadcopter_scan(trees, visible_from_outside);

    visibility.iter().map(|row| row.iter().sum::<u32>()).sum()
}

fn part_two(trees: Grove) -> u32 {
    let visibility = quadcopter_scan(trees, scenic_score);

    visibility
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap()
        .clone()
}

fn quadcopter_scan(trees: Grove, score_fn: fn(&Grove, Coord) -> u32) -> Map<u32> {
    let mut visibility = vec![vec![0; trees[0].len()]; trees.len()];

    for (r, row) in trees.iter().enumerate() {
        for (c, _col) in row.iter().enumerate() {
            visibility[r][c] = score_fn(&trees, (r, c));
        }
    }

    visibility
}

fn visible_from_outside(trees: &Grove, (r, c): Coord) -> u32 {
    if r == 0 || c == 0 || r == trees.len() - 1 || c == trees[0].len() - 1 {
        return 1;
    }

    let (row, col, tree) = row_column_and_tree(trees, (r, c));

    let left = row[0..c].iter().max().unwrap() < &tree;
    let right = row[c + 1..].iter().max().unwrap() < &tree;
    let top = col[0..r].iter().max().unwrap() < &tree;
    let bottom = col[r + 1..].iter().max().unwrap() < &tree;

    (left || right || top || bottom) as u32
}

fn scenic_score(trees: &Grove, (r, c): Coord) -> u32 {
    let (row, col, tree) = row_column_and_tree(trees, (r, c));

    let left = count_until_blocked(row[0..c].to_vec().into_iter().rev().collect(), tree);
    let right = count_until_blocked(row[c + 1..].to_vec(), tree);
    let top = count_until_blocked(col[0..r].to_vec().into_iter().rev().collect(), tree);
    let bottom = count_until_blocked(col[r + 1..].to_vec(), tree);

    (left * right * top * bottom) as u32
}

fn row_column_and_tree(trees: &Grove, (r, c): Coord) -> (Vec<Tree>, Vec<Tree>, Tree) {
    let row = trees[r].clone();
    let col = trees
        .iter()
        .map(|row| row.iter().nth(c).unwrap().clone())
        .collect();
    let tree = trees[r][c];

    (row, col, tree)
}

fn count_until_blocked(trees: Vec<Tree>, treehouse: Tree) -> usize {
    let view = trees.iter().take_while(|&&t| t < treehouse).count();

    // count the blocking tree, if not edge
    if view < trees.len() {
        return view + 1;
    } else {
        return view;
    }
}

fn trees(path: &str) -> Grove {
    fs::read_to_string(path)
        .expect("That is not a valid input")
        .lines()
        .map(|row| row.chars().map(|tree| tree.to_digit(10).unwrap()).collect())
        .collect()
}

fn main() {
    let path = "day-08/data/input.txt";
    let trees = trees(path);
    println!("Part one: {}", part_one(trees.clone()));
    println!("Part two: {}", part_two(trees.clone()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let path = "data/test.txt";
        let trees = trees(path);
        assert_eq!(part_one(trees), 21);
    }

    #[test]
    fn test_part_two() {
        let path = "data/test.txt";
        let trees = trees(path);
        assert_eq!(part_two(trees), 8);
    }
}
