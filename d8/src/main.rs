use std::fs;

fn main() {
    let trees = fs::read_to_string("input.txt".to_string()).expect("Unable to read");

    let grid = build_grid(trees);

    let mut visible_trees = 0;

    let mut scenic_scores: Vec<i32> = Vec::new();

    for i in 0..(grid.len()) {
        for j in 0..(grid[0].len()) {
            if is_visible(&grid, i, j) {
                visible_trees += 1;
            }

            scenic_scores.push(get_scenic_score(&grid, i, j))
        }
    }

    println!("Visible trees: {}", visible_trees);

    let mut max_scenic_score = 0;

    for score in scenic_scores.iter() {
        if score > &max_scenic_score {
            max_scenic_score = score.clone();
        }
    }

    println!("Max scenic score: {}", max_scenic_score);
}

fn build_grid(trees: String) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in trees.lines() {
        let mut treeline = Vec::new();
        for tree in line.chars() {
            treeline.push(tree.to_digit(10).unwrap() as u8)
        }
        grid.push(treeline)
    }
    grid
}

fn get_sides(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut left = grid[i][..j].to_vec();
    left.reverse();
    let right = grid[i][(j + 1)..].to_vec();
    let mut top = Vec::new();
    let mut bottom = Vec::new();

    for t in 0..i {
        top.push(grid[t][j]);
    }
    top.reverse();

    for b in (i + 1)..grid.len() {
        bottom.push(grid[b][j]);
    }

    (left, right, top, bottom)
}

fn is_visible(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    let tree = &grid[i][j];
    let (left, right, top, bottom) = get_sides(&grid, i, j);

    for side in vec![&left, &right, &top, &bottom] {
        if side.iter().all(|x| x < tree) {
            return true;
        }
    }
    false
}

fn get_scenic_score(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> i32 {
    let tree = &grid[i][j];
    let (left, right, top, bottom) = get_sides(&grid, i, j);
    let mut scenic_score = 1;

    for side in vec![&left, &right, &top, &bottom] {
        let mut side_amt = 0;
        if side.len() == 0 {
            scenic_score *= side_amt;
            break;
        }
        for neighbor in side.iter() {
            if neighbor < tree {
                side_amt += 1;
            } else {
                side_amt += 1;
                break;
            }
        }
        scenic_score *= side_amt;
    }
    scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenic_score() {
        let trees = fs::read_to_string("test.txt").unwrap();

        let grid = build_grid(trees);

        assert_eq!(4, get_scenic_score(&grid, 1, 2));
        assert_eq!(8, get_scenic_score(&grid, 3, 2));
    }
}
