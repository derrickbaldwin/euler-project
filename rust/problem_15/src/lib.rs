/*

Problem 15: 

Starting in the top left corner of a 2×2 grid, and 
only being able to move to the right and down, there 
are exactly 6 routes to the bottom right corner.

How many such routes are there through a 20×20 grid?

This is combination with no repetition: 

where n is the number of things to choose from, 
and we choose r of them, no repetition, order doesn't 
matter. 40 choose 20.

We use a (factored) modified formula to avoid factorial 
overflows 

*/
//pub fn lattice
pub fn lattice_routes(grid_size: u64) -> u64 {
    choose(2 * grid_size, grid_size)
} 

fn choose(n: u64, k: u64)  -> u64 {
   let mut num = 1;
   let mut den = 1;
   for i in 0..k {
       num *= n - i;
       den *= i + 1; 
       if num % den == 0 {
           num = num / den;
           den  = den / den; 
       }
   }
   num / den   
}


#[cfg(test)]
mod tests {
    use lattice_routes;
    
    #[test]
    fn number_of_paths_1() {
        assert_eq!(lattice_routes(1), 2);
    }

    #[test]
    fn number_of_paths_2() {
        assert_eq!(lattice_routes(2), 6);
    }
}
