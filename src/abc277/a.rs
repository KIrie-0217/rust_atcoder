use proconio::input;

fn main(){

    input!{
        n:usize,
        _x:usize,
        mut p:[ usize; n]
    }
    
    println!( "{}", 1 as usize + p.iter().position( |&x| x == _x ).unwrap());
}