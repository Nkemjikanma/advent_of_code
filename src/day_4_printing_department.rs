use std::fs::File;
use std::io::{self, BufRead, BufReader};
pub fn get_accessible_rolls() {
    println!("-------- Day 4 --------");
    let content = File::open("day_4_printing_dept.txt").unwrap();

    let lines = BufReader::new(content).lines();

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in lines.map_while(io::Result::ok) {
        if line.is_empty() {
            continue;
        }

        let row: Vec<char> = line.chars().collect();

        grid.push(row);
    }

    // println!("{:?}", grid.len());
    //

    let mut total_valid_paper_roll = 0;
    let mut valid_paper_per_loop = true;
    let mut valid_rolls_to_remove: Vec<(usize, usize)> = Vec::new();

    while valid_paper_per_loop {
        if valid_rolls_to_remove.len() != 0 {
            println!("{}", valid_rolls_to_remove.len());

            for &(r, c) in &valid_rolls_to_remove {
                grid[r][c] = 'x';
            }

            valid_rolls_to_remove.clear();
        }

        valid_paper_per_loop = false;

        for row_idx in 0..grid.len() {
            'griditem: for col_idx in 0..grid[row_idx].len() {
                if grid[row_idx][col_idx] != '@' {
                    continue 'griditem;
                }
                let mut eight_adjacent_rows_sum = 0;

                let adjacent_positions: [(isize, isize); 8] = [
                    (row_idx as isize - 1, col_idx as isize - 1),
                    (row_idx as isize, col_idx as isize - 1),
                    (row_idx as isize + 1, col_idx as isize - 1),
                    (row_idx as isize - 1, col_idx as isize),
                    (row_idx as isize + 1, col_idx as isize),
                    (row_idx as isize - 1, col_idx as isize + 1),
                    (row_idx as isize, col_idx as isize + 1),
                    (row_idx as isize + 1, col_idx as isize + 1),
                ];

                for &(position_row, position_col) in &adjacent_positions {
                    if position_row < 0 || position_col < 0 {
                        continue;
                    }

                    // type casting to allow for indexing
                    let (r, c) = (position_row as usize, position_col as usize);

                    if r >= grid.len() || c >= grid[r].len() {
                        continue;
                    }

                    if grid[r][c] == '@' {
                        eight_adjacent_rows_sum += 1;
                    }
                }

                if eight_adjacent_rows_sum < 4 {
                    total_valid_paper_roll += 1;

                    valid_rolls_to_remove.push((row_idx, col_idx));
                } else if eight_adjacent_rows_sum >= 4 {
                    continue;
                }
            }
        }

        if !valid_rolls_to_remove.is_empty() {
            valid_paper_per_loop = true;
        }
        if !valid_paper_per_loop {
            break;
        }
    }

    println!("valid_paper_roll = {}", total_valid_paper_roll);
}
