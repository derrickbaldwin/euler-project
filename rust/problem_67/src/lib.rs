use std::cmp;

fn pad_zeros(t: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = t.len();
    let mut grid = vec![vec![0; n]; n]; 
    for i in 0..n {
        for (j,_) in t[i].iter().enumerate() {
            grid[i][j] = t[i][j];
        }
    }
    grid
}

pub fn max_path(triangle: &Vec<Vec<i32>>) -> i32 {
    let n = triangle.len();
    let mut grid = pad_zeros(triangle);
    
    for i in (0 .. n-1).rev() {
        for j in 0 .. n-1 {
            grid[i][j] += cmp::max(grid[i + 1][j], 
                                   grid[i + 1][j + 1]);
        }
    }
    grid[0][0]
}


#[cfg(test)]
mod tests {
    use max_path;

    
    #[test]
    fn it_works() {
        let test = [ [3].to_vec(), 
                     [7, 4].to_vec(), 
                     [2, 4, 6].to_vec(), 
                     [8, 5, 9, 3].to_vec()
                   ].to_vec();
        assert_eq!(max_path(&test), 23);
    }
}