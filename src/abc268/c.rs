use proconio::input;
use itertools::Itertools;

fn main(){

    input!{
        n:usize,
        p:[ usize ; n]
    }

    println!("{:?}", p);
}