use proconio::input;
use itertools::Itertools;

fn main(){

    input!{
        n:i32,
        m:i32
    }
    
    for comb in (1..m+1).combinations(n as usize){
        
        for i in comb.iter(){
            print!("{} ", i);
        }
        println!();
    }

}