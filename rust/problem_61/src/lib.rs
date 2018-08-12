/*

Problem 61:


Triangle, square, pentagonal, hexagonal, heptagonal, and octagonal 
numbers are all figurate (polygonal) numbers and are generated by 
the following formulae:

Triangle 	  	P3,n=n(n+1)/2 	  	1, 3, 6, 10, 15, ...
Square 	  	    P4,n=n2 	  	1, 4, 9, 16, 25, ...
Pentagonal 	  	P5,n=n(3n−1)/2 	  	1, 5, 12, 22, 35, ...
Hexagonal 	  	P6,n=n(2n−1) 	  	1, 6, 15, 28, 45, ...
Heptagonal 	  	P7,n=n(5n−3)/2 	  	1, 7, 18, 34, 55, ...
Octagonal 	  	P8,n=n(3n−2) 	  	1, 8, 21, 40, 65, ...

The ordered set of three 4-digit numbers: 8128, 2882, 8281, has 
three interesting properties.

    The set is cyclic, in that the last two digits of each number is 
    the first two digits of the next number (including the last number 
    with the first).
    Each polygonal type: triangle (P3,127=8128), square (P4,91=8281), 
    and pentagonal (P5,44=2882), is represented by a different number 
    in the set. This is the only set of 4-digit numbers with 
    this property.

Find the sum of the only ordered set of six cyclic 4-digit numbers 
for which each polygonal type: triangle, square, pentagonal, hexagonal, 
heptagonal, and octagonal, is represented by a different number 
in the set.
*/


#[derive(Debug, Clone, Hash)]
pub struct Figurate {
    pub left: u64,
    pub right: u64,
} 

impl From<u64> for Figurate {
    fn from(n: u64) -> Self {
        Figurate {
            left: n / 100,
            right: n % 100
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Polygon {
    pub shape: u8,
    pub figurate: Figurate,
    pub num: u64,
}

pub fn polygons<F>(fns: Vec<F>) ->  Vec<Polygon> 
where F: Fn(u64) -> Polygon {
    let mut v = Vec::new();
    for f in fns {
        let x = (21..141).map(|x| f(x))
                         .collect::<Vec<_>>()
                         .into_iter()
                         .filter(|x| x.num > 1000 && x.num < 9999)
                         .collect::<Vec<_>>();
        v.extend(x);
    }
    v
} 

pub fn triangle(n: u64) -> Polygon {
    let x = n * (n + 1) / 2;
    Polygon {
        shape: 3, 
        figurate: Figurate::from(x),
        num: x 
    }
}

pub fn square(n: u64) -> Polygon {
    let x = n * n;
    Polygon {
        shape: 4,
        figurate: Figurate::from(x),
        num: x
    }
}

pub fn pentagonal(n: u64) -> Polygon {
    let x = n * (3 * n - 1) / 2;
    Polygon {
        shape: 5,
        figurate: Figurate::from(x),
        num: x
    }
}

pub fn hexagonal(n: u64) -> Polygon {
    let x = n * (2 * n - 1);
    Polygon {
        shape: 6,
        figurate: Figurate::from(x),
        num: x
    }
}

pub fn heptagonal(n: u64) -> Polygon {
    let x = n * (5 * n - 3) / 2;
    Polygon {
        shape: 7,
        figurate: Figurate::from(x),
        num: x
    }
}

pub fn octagonal(n: u64) -> Polygon {
    let x = n * (3 * n - 2);
    Polygon {
        shape: 8,
        figurate: Figurate::from(x), 
        num: x 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_works() {
        let triangles = (1..6).map(|x| triangle(x)).collect::<Vec<_>>();
        let nums = triangles.iter().map(|x| x.num).collect::<Vec<_>>(); 
        assert_eq!(nums, vec!(1, 3, 6, 10, 15));
    }

    #[test]
    fn square_works() {
        let squares = (1..6).map(|x| square(x)).collect::<Vec<_>>();
        let nums = squares.iter().map(|x| x.num).collect::<Vec<_>>();
        assert_eq!(nums, vec!(1, 4, 9, 16, 25));
    }

    
    #[test]
    fn pentagonal_works() {
        let pents = (1..6).map(|x| pentagonal(x)).collect::<Vec<_>>();
        let nums = pents.iter().map(|x| x.num).collect::<Vec<_>>();
        assert_eq!(nums, vec!(1, 5, 12, 22, 35));
    }

    #[test]
    fn hexagonal_works() {
        let hexs = (1..6).map(|x| hexagonal(x)).collect::<Vec<_>>();
        let nums = hexs.iter().map(|x| x.num).collect::<Vec<_>>();
        assert_eq!(nums, vec!(1, 6, 15, 28, 45));
    }

    #[test]
    fn heptagonal_works() {
        let hepts = (1..6).map(|x| heptagonal(x)).collect::<Vec<_>>();
        let nums = hepts.iter().map(|x| x.num).collect::<Vec<_>>();
        assert_eq!(nums, vec!(1, 7, 18, 34, 55));
    }

    #[test]
    fn octagonal_works() {
        let octs = (1..6).map(|x| octagonal(x)).collect::<Vec<_>>();
        let nums = octs.iter().map(|x| x.num).collect::<Vec<_>>();
        assert_eq!(nums, vec!(1, 8, 21, 40, 65));
    }

    #[test]
    fn triangle_127() {
        let x = triangle(127);
        assert_eq!(x.num, 8128);
    }

    
    #[test]
    fn square_91() {
        let x = square(91);
        assert_eq!(x.num, 8281);
    }

    #[test]
    fn pentagonal_44() {
        let x = pentagonal(44);
        assert_eq!(x.num, 2882);
    }

}
