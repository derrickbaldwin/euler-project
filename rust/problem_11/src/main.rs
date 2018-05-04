extern crate problem_11;

use std::fs::File;
use std::io::prelude::*;

pub fn transpose(square_matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let rows = square_matrix.len();
    let cols = square_matrix[0].len();
    let mut m: Vec<Vec<u32>> = vec![vec![0;20]; 20];
    for i in 0..rows {
        for j in 0..cols {
            m[i][j] = square_matrix[j][i];
        }
    }
    m
}

fn main() {
    let mut f = File::open("grid.txt").expect("File not found");
    let mut nums = String::new();
    f.read_to_string(&mut nums).expect("Could not read file");
     // prepare grid
    let grid: Vec<Vec<u32>> = nums.lines()
                                  .map(|row| {
                                      row.split_whitespace()
                                         .filter_map(|d| d.parse().ok())
                                         .collect()
                                  })
                                  .collect();
    // hold max searches    
    let mut maxs = Vec::new();

    let mut max_rows = 0;
    for i in 0..20 {
        for j in 0..17 {
            let mut prod = grid[i][j] * grid[i][j + 1] * 
                        grid[i][j + 2] * grid[i][j + 3];
            if prod > max_rows {
                max_rows = prod;
            }
        }
    }
    maxs.push(max_rows);

    let grid_trans = transpose(&grid);
    let mut max_cols = 0;
    for i in 0..20 {
        for j in 0..17 {
            let mut prod = grid_trans[i][j] * grid_trans[i][j + 1] * 
                        grid_trans[i][j + 2] * grid_trans[i][j + 3];
            if prod > max_cols {
                max_cols = prod;
            }
        }
    }
    maxs.push(max_cols);

    let mut max_diagonal_from_left = 0;
    for i in 0..17 { 
        for j in 0..17 {
            let mut prod = grid[i][j] * grid[i + 1][j + 1] * 
                        grid[i + 2][j + 2] * grid[i + 3][j + 3];
            if prod > max_diagonal_from_left {
                max_diagonal_from_left = prod;
            }
        }
    }

    maxs.push(max_diagonal_from_left);
    let mut max_diagonal_from_right = 0;
    for i in 3..20 { 
        for j in 0..17 {
            let mut prod = grid[i][j] * grid[i - 1][j + 1] * 
                        grid[i - 2][j + 2] * grid[i - 3][j + 3];
            if prod > max_diagonal_from_right {
                max_diagonal_from_right = prod;
            }
        }
    }
    maxs.push(max_diagonal_from_right);

    println!("{:?}", maxs.iter().max().unwrap())
}