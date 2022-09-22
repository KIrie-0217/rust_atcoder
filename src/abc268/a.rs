use proconio::input;
use std::collections::HashSet;
fn main(){

    input!{
        abcde:[ usize; 5],
    }

    let mut hash = HashSet::new();
    for i in abcde.iter(){
        hash.insert( i.clone());
    }

    println!("{:?}", hash.len() );


}