use proconio::input;
use itertools::Itertools;

fn main(){

    input!{
        n:usize,
        k:usize,
        d:usize,
        mut a_vec: [ usize ; n]
    }
    
    let mut ans : usize = 0;
    a_vec.reverse();

    println!("{:?}", a_vec);

}