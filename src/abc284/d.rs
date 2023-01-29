use itertools::Itertools;
use num_integer::Roots;
use proconio::input;

fn prime_find(target: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::<usize>::new();

    if target % 2 == 0 {
        res.push(2);
    }
    
    for i in (3..target.sqrt()).step_by(2) {}
}

fn main() {
    input! {
        tests:usize
    }

    for _ in 0..tests {
        input! {
            target:usize
        }
    }
}
