use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};
use std::iter::repeat;

fn solution_part1(grid: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    let rows = grid.len();
    let columns = grid[0].len();

    for row_idx in 1..(rows - 1) {
        // from left to right
        let mut max_in_row = grid[row_idx][0];
        for column_idx in 1..(columns - 1) {
            if grid[row_idx][column_idx] > max_in_row {
                visible_trees.insert((row_idx, column_idx));
                max_in_row = grid[row_idx][column_idx];
            }
        }

        // from right to left
        let mut max_in_row = grid[row_idx][columns - 1];
        for column_idx in (1..(columns - 1)).rev() {
            if grid[row_idx][column_idx] > max_in_row {
                visible_trees.insert((row_idx, column_idx));
                max_in_row = grid[row_idx][column_idx];
            }
        }
    }

    // Go column by column
    for column_idx in 1..(columns - 1) {
        // from up to bottom
        let mut max_in_column = grid[0][column_idx];
        for row_idx in 1..(rows - 1) {
            if grid[row_idx][column_idx] > max_in_column {
                visible_trees.insert((row_idx, column_idx));
                max_in_column = grid[row_idx][column_idx];
            }
        }

        // from bottom to up
        let mut max_in_column = grid[rows - 1][column_idx];
        for row_idx in (1..(rows - 1)).rev() {
            if grid[row_idx][column_idx] >  max_in_column {
                visible_trees.insert((row_idx, column_idx));
                max_in_column = grid[row_idx][column_idx];
            }
        }
    }

    return visible_trees;
}

fn scenic_score(grid: &Vec<Vec<u8>>, row_idx: usize, column_idx: usize) -> u64 {
    let mut up_score = 0;
    let tree_height = grid[row_idx][column_idx];

    for row_scene_idx in (0..row_idx).rev() {
        up_score += 1;

        if grid[row_scene_idx][column_idx] >= tree_height {
            break;
        }
    }

    let mut down_score = 0;

    for row_scene_idx in (row_idx + 1)..grid.len() {
        down_score += 1;

        if grid[row_scene_idx][column_idx] >= tree_height {
            break;
        }
    }

    let mut left_score = 0;

    for column_scene_idx in (0..column_idx).rev() {
        left_score += 1;

        if grid[row_idx][column_scene_idx] >= tree_height {
            break;
        }
    }

    let mut right_score = 0;

    for column_scene_idx in (column_idx + 1)..grid[0].len() {
        right_score += 1;

        if grid[row_idx][column_scene_idx] >= tree_height {     
            break;
        }
    }

    return up_score * down_score * left_score * right_score;
}

fn solution_part2(grid: &Vec<Vec<u8>>) -> u64 {
    let rows = grid.len();
    let columns = grid[0].len();

    let max_scenic_score = (1..(rows - 1))
        .flat_map(|row_idx| {
            let repeated_row_idx = repeat(row_idx); 
            return repeated_row_idx.zip(1..(columns - 1));
        })
        .map(|(row_idx, column_idx)| scenic_score(grid, row_idx, column_idx))
        .max();
    
    return max_scenic_score.unwrap_or(0);
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let grid: Vec<Vec<u8>> = reader.lines()
        .map(|line| line.unwrap()
            .trim_end()
            .bytes()
            .map(|c| c - b'0')
            .collect())
        .collect();
        
    // let visible_trees = solution_part1(&grid);
    // let result = 2*grid.len() + 2*(grid[0].len() - 2) + visible_trees.len();

    let result = solution_part2(&grid);

    println!("Answer: {}", result);
}
