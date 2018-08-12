extern crate problem_61;

use problem_61::*;


fn main() {
    println!("{}", "Problem 61");

    let octals = (19..59).map(|x| octagonal(x)).collect::<Vec<_>>();
    let other_figurates = polygons([triangle, square, pentagonal, hexagonal, heptagonal].to_vec());
    
    for mut n1 in octals {
        for mut n2 in &other_figurates {
            if n1.shape != n2.shape && n1.figurate.right == n2.figurate.left {
                for n3 in &other_figurates {
                    if n1.shape != n3.shape && n2.shape != n3.shape && 
                            n2.figurate.right == n3.figurate.left {
                        for n4 in &other_figurates {
                            if n1.shape != n4.shape && n2.shape != n4.shape && 
                                    n3.shape != n4.shape && n3.figurate.right == n4.figurate.left {
                                for n5 in &other_figurates {
                                    if n1.shape != n5.shape && n2.shape != n5.shape && 
                                            n3.shape != n5.shape && n4.shape != n5.shape && 
                                            n4.figurate.right == n5.figurate.left {
                                        for n6 in &other_figurates {
                                            if n1.shape != n6.shape && n2.shape != n6.shape && 
                                                n3.shape != n6.shape && n4.shape != n6.shape && 
                                                n5.shape != n6.shape && 
                                                n5.figurate.right == n6.figurate.left && 
                                                n6.figurate.right == n1.figurate.left {
                                                println!("{:?}", n1.num + n2.num + n3.num 
                                                              + n4.num + n5.num + n6.num);                                                
                                            }
                                        }                                        
                                    }
                                }
                            }
                       }
                    }
                }
            }
        }
    }
}

