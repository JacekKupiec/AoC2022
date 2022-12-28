use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

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
    
    let rows = grid.len();
    let columns = grid[0].len();
    let mut visible_trees: HashSet<(usize, usize)>= HashSet::new();

    // Go row by row
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
    
    let result = 2*rows + 2*(columns - 2) + visible_trees.len();
    println!("Answer: {}", result);
}
