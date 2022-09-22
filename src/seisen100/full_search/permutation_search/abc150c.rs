use proconio::input;
use itertools::Itertools;

fn main(){

    input!{
        n:usize,
        p:[ usize ; n],
        q:[ usize ; n]

    }

    let mut p_num:i64= 0;
    let mut q_num:i64= 0;

    for ( count , i) in (1..n+1).permutations(n).enumerate(){

        if i == p{
            p_num = count as i64;
        }
        if i == q{
            q_num = count as i64;
        }
    }

    println!("{}",(p_num - q_num).abs());
}